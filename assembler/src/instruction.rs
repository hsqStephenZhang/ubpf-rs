use std::mem;

use nom::AsBytes;

use crate::{
    alu,
    assemble::asm::assemble,
    class,
    error::ParseError,
    utils::{imm, memory, offset, reg},
};

#[derive(Clone, Copy)]
pub struct Instruction {
    pub op: u8,
    pub regs: u8,
    pub offset: i16,
    pub imm: i32,
    pub next_imm: Option<i32>,
}

impl Instruction {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Instruction {
        let mut op = 0;
        let mut regs = 0;
        let mut offset = 0;
        let mut imm = 0;
        // if current instruction is `lddx`, we should also load the next instruction's imm
        let mut next_imm = None;

        unsafe {
            let (mut src, _): (usize, usize) = mem::transmute(bytes);
            std::ptr::copy(src as *const u8, &mut op as *mut _ as _, 1);
            src += 1;
            std::ptr::copy(src as *const u8, &mut regs as *mut _ as _, 1);
            src += 1;
            std::ptr::copy(src as *const u8, &mut offset as *mut _ as _, 2);
            src += 2;
            std::ptr::copy(src as *const u8, &mut imm as *mut _ as _, 4);
            src += 8;

            if op == 0x18 {
                let mut tmp = 0;
                std::ptr::copy(src as *const u8, &mut tmp as *mut _ as _, 4);
                next_imm = Some(tmp);
            }
        }

        // dbg!(op, regs, offset, imm, next_imm);

        Self {
            op,
            regs,
            offset,
            imm,
            next_imm: next_imm,
        }
    }

    #[inline(always)]
    pub fn new(op: u8, regs: u8, offset: i16, imm: i32) -> Instruction {
        Self {
            op,
            regs,
            offset,
            imm,
            next_imm: None,
        }
    }

    #[inline(always)]
    pub fn src_reg(&self) -> u8 {
        (self.regs >> 4) & 0xf
    }

    #[inline(always)]
    pub fn dst_reg(&self) -> u8 {
        self.regs & 0xf
    }

    #[inline(always)]
    pub fn class(&self) -> u8 {
        self.op & 0x7
    }

    // for alu/alu64/jmp
    #[inline(always)]
    pub fn opcode(&self)-> u8 {
        (self.op >> 4) & 0xf // 0x11110000
    }

    // for alu/alu64/jmp
    #[inline(always)]
    pub fn source(&self) -> u8 {
        (self.op >> 3) & 0x1 // 0x00001000
    }
}

impl std::fmt::Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cls = self.class();
        let dst = self.dst_reg();
        let src = self.src_reg();

        if cls == class::EBPF_CLS_ALU || cls == class::EBPF_CLS_ALU64 {
            let source = self.source(); // 0x00001000
            let opcode = self.opcode(); // 0x11110000
            let opcode_name = *crate::ALU_OPCODES_TO_NAME.get(&opcode).unwrap();

            match opcode {
                alu::END => {
                    let opcode_name = if source == 1 { "be" } else { "le" };
                    return write!(f, "{}{} {}", opcode_name, self.imm, reg(self.dst_reg()));
                }
                alu::NEG => {
                    return write!(f, "{} {}", opcode_name, reg(dst));
                }
                _ => {
                    if source == 0 {
                        return write!(f, "{} {}, {}", opcode_name, reg(dst), imm(self.imm),);
                    } else {
                        return write!(f, "{} {}, {}", opcode_name, reg(dst), reg(src),);
                    }
                }
            }
        } else if cls == class::EBPF_CLS_JMP {
            let source = self.source(); // 0x00001000
            let opcode = self.opcode(); // 0x11110000
            let opcode_name = *crate::JMP_OPCODES_TO_NAME.get(&opcode).unwrap();
            match opcode_name {
                "exit" => return write!(f, "{}", opcode_name),
                "call" => return write!(f, "{} {}", opcode_name, imm(self.imm)),
                "ja" => return write!(f, "{} {}", opcode_name, offset(self.offset)),
                _ => {
                    if source == 0 {
                        return write!(
                            f,
                            "{} {},{}, {}",
                            opcode_name,
                            reg(dst),
                            imm(self.imm),
                            offset(self.offset)
                        );
                    } else {
                        return write!(
                            f,
                            "{} {},{}, {}",
                            opcode_name,
                            reg(dst),
                            imm(src as i32),
                            offset(self.offset)
                        );
                    }
                }
            }
        } else if cls == class::EBPF_CLS_LD
            || cls == class::EBPF_CLS_LDX
            || cls == class::EBPF_CLS_ST
            || cls == class::EBPF_CLS_STX
        {
            let size = (self.op >> 3) & 0x3; // 0x00011000
            let size_name = *crate::SIZES_TO_NAME.get(&size).unwrap();
            let class_name = *crate::CLASS.get(&cls).unwrap();

            // handle special cases: lddx
            if let Some(next_imm) = self.next_imm {
                let double_word_imm = ((next_imm as i64) << 32) + self.imm as i64;
                return write!(
                    f,
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    reg(dst),
                    double_word_imm
                );
            } else if self.op == 0x0 {
                // section instruction of lddw
                return write!(f, "");
            }

            match cls {
                class::EBPF_CLS_LDX => {
                    write!(
                        f,
                        "{}{} {}, {}",
                        class_name,
                        size_name,
                        reg(dst),
                        memory(&reg(src), self.offset)
                    )
                }
                class::EBPF_CLS_ST => {
                    write!(
                        f,
                        "{}{} {}, {}",
                        class_name,
                        size_name,
                        memory(&reg(dst), self.offset),
                        imm(self.imm),
                    )
                }
                class::EBPF_CLS_STX => {
                    write!(
                        f,
                        "{}{} {}, {}",
                        class_name,
                        size_name,
                        memory(&reg(dst), self.offset),
                        reg(src),
                    )
                }
                _ => write!(f, "unknown instruction {}", self.op),
            }
        } else {
            unreachable!("{:?}", cls)
        }
    }
}

#[derive(Clone)]
pub struct Instructions {
    inner: Vec<Instruction>,
}

impl Instructions {
    pub fn new(inner: Vec<Instruction>) -> Self {
        Self { inner }
    }

    pub fn from_asm(text: &str) -> Result<Self, ParseError> {
        let inner = assemble(text)?;
        Ok(Self { inner })
    }

    pub fn into_vec(self)->Vec<Instruction> {
        self.inner
    }
}

impl From<&[u8]> for Instructions {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len() % 8, 0);
        let num_ins = bytes.len() >> 3;
        let mut inner = Vec::with_capacity(num_ins);

        for i in (0..bytes.len()).into_iter().step_by(8) {
            let r = Instruction::from_bytes(&bytes[i..]);
            inner.push(r);
        }

        Self { inner }
    }
}

impl From<Vec<u8>> for Instructions {
    fn from(bytes: Vec<u8>) -> Self {
        Self::from(bytes.as_bytes())
    }
}

impl std::fmt::Debug for Instructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, ins) in self.inner.iter().enumerate() {
            write!(f, "{}:{:?}\n", index, ins)?
        }
        Ok(())
    }
}
