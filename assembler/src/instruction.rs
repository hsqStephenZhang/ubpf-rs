use std::mem;

use nom::AsBytes;

use crate::{
    alu,
    assemble::asm::assemble,
    class,
    error::ParseError,
    utils::{memory, reg},
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
        // FIXME: not sure if we should load next the instruction when facing lddx
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
}

impl Into<Vec<Instruction>> for Instructions {
    fn into(self) -> Vec<Instruction> {
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

impl From<Vec<Instruction>> for Instructions {
    fn from(instructions: Vec<Instruction>) -> Self {
        Self {
            inner: instructions,
        }
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

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn t1() {
        let ins = [
            0xb7, 0x01, 0x00, 0x00, 0x72, 0x6c, 0x64, 0x0a, // r1 = 174353522
            0x63, 0x1a, 0xf8, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u32 *)(r10 - 8) = r1
            0x18, 0x01, 0x00, 0x00, 0x48, 0x65, 0x6c, 0x6c, //
            0x00, 0x00, 0x00, 0x00, 0x6f, 0x20, 0x57, 0x6f, // r1 = 8022916924116329800 ll
            0x7b, 0x1a, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u64 *)(r10 - 16) = r1
            0xb7, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = 0
            0x73, 0x1a, 0xfc, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u8 *)(r10 - 4) = r1
            0xbf, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = r10
            0x07, 0x01, 0x00, 0x00, 0xf0, 0xff, 0xff, 0xff, // r1 += -16
            0xb7, 0x02, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, // r2 = 13
            0x85, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // call 6
            0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r0 = 0
            0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit
        ];
        /*
                 mov r1,#174353522
                 stxw r10-8, r1
                 lddw r1, #1819043144
                 stxdw r10-16, r1
                 mov r1,#0
                 stxb r10-4, r1
                 mov r1,r10
                 add r1,#-16
                 mov r2,#13
                 call #6
                 mov r0,#0
                 exit
        */
        let instructions = Instructions::from(ins.as_slice());
        println!("{:?}", instructions);
        // let a=1819043144;
        // let b=
        // println!("instruction:{}", r);
    }

    #[test]
    fn t2() {
        let ins = [
            0xbf, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r2 = r10
            0x07, 0x02, 0x00, 0x00, 0xfc, 0xff, 0xff, 0xff, // r2 += -4
            0x18, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, //
            0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // lddw r1, #4294967296
            0x15, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // if r0 == 0 goto +3 <LBB0_20>
            0x79, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = *(u64 *)(r0 + 0)
            0x07, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // r1 += 1
            0x7b, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // *(u64 *)(r0 + 0) = r1
        ];

        /*
           mov r2,r10
           add r2,#-4
           lddw r1, #4294967296
           call #1
           jeq r0,#0, 3
           ldxdw r1, r0
           add r1,#1
           stxdw r0, r1
        */
        let instructions = Instructions::from(ins.as_slice());
        println!("{:?}", instructions);
    }
}
