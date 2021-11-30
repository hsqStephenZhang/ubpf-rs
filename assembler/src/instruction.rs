use std::mem;

use nom::AsBytes;

use crate::{
    alu,
    assemble::asm::assemble,
    class,
    error::ParseError,
    utils::{memory, reg}, JitBuilder,
};

const LDDW: u8 = 0x18;
const INS_SIZE: usize = 8;

#[derive(Clone, Copy)]
pub struct Instruction {
    pub op: u8,
    pub regs: u8,
    pub offset: i16,
    pub imm: i64,
}

impl Instruction {
    #[inline(always)]
    pub fn new(op: u8, regs: u8, offset: i16, imm: i64) -> Instruction {
        Self {
            op,
            regs,
            offset,
            imm,
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
    pub fn opcode(&self) -> u8 {
        (self.op >> 4) & 0xf // 0x11110000
    }

    // for alu/alu64/jmp
    #[inline(always)]
    pub fn source(&self) -> u8 {
        (self.op >> 3) & 0x1 // 0x00001000
    }
}

/// from bytes, notice that we should handle lddw here, which means
/// we should perceive the next instruction's raw content
impl From<&[u8]> for Instruction {
    fn from(bytes: &[u8]) -> Self {
        let mut op = 0;
        let mut regs = 0;
        let mut offset = 0;
        // if current instruction is `lddx`, we should also load the next instruction's imm
        let mut imm = 0;

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
                assert!(bytes.len() >= 16);
                let mut tmp = 0;
                std::ptr::copy(src as *const u8, &mut tmp as *mut _ as _, 4);
                imm = ((tmp as i64) << 32) | imm;
            }

            dbg!(imm);
        }

        Self {
            op,
            regs,
            offset,
            imm,
        }
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
                        return write!(f, "{} {}, {}", opcode_name, reg(dst), self.imm,);
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
                "call" => return write!(f, "{} {}", opcode_name, self.imm),
                "ja" => return write!(f, "{} {}", opcode_name, self.offset),
                _ => {
                    if source == 0 {
                        return write!(
                            f,
                            "{} {},{}, {}",
                            opcode_name,
                            reg(dst),
                            self.imm,
                            self.offset
                        );
                    } else {
                        return write!(
                            f,
                            "{} {},{}, {}",
                            opcode_name,
                            reg(dst),
                            src as i64,
                            self.offset
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
            if self.op == 0x18 {
                return write!(f, "{}{} {}, {}", class_name, size_name, reg(dst), self.imm);
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
                        self.imm,
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

    pub fn into_vec(self) -> Vec<Instruction> {
        self.inner
    }
}

impl From<&[u8]> for Instructions {
    fn from(bytes: &[u8]) -> Self {
        assert_eq!(bytes.len() % INS_SIZE, 0);
        let num_ins = bytes.len() >> 3;
        let mut inner = Vec::with_capacity(num_ins);

        let mut i = 0;

        while i < bytes.len() {
            let r = Instruction::from(&bytes[i..]);
            inner.push(r);
            if r.op == LDDW {
                i += INS_SIZE;
            }
            i += INS_SIZE;
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

impl Instructions{
    pub fn jit(&self)->JitBuilder{
        let a=JitBuilder::new();
        a
    }
}