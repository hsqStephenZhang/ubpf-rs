mod asm_parser;
#[allow(dead_code)]
mod assemble;
mod ebpf;
mod disassemble;
mod error;
mod utils;

use std::{collections::HashMap, mem};

// pub use assemble::*;
pub use ebpf::{alu, class, op};
pub use disassemble::{disassemble, disassemble_one};
pub use error::ElfError;
pub use utils::*;

lazy_static::lazy_static! {
    pub static ref CLASS: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "ld");
        m.insert(1, "ldx");
        m.insert(2, "st");
        m.insert(3, "stx");
        m.insert(4, "alu");
        m.insert(5, "jmp");
        m.insert(7, "alu64");
        m
    };

    pub static ref ALU_OPCODES: HashMap<u8, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "add");
        m.insert(1, "sub");
        m.insert(2, "mul");
        m.insert(3, "div");
        m.insert(4, "or");
        m.insert(5, "and");
        m.insert(6, "lsh");
        m.insert(7, "rsh");
        m.insert(8, "neg");
        m.insert(9, "mod");
        m.insert(10, "xor");
        m.insert(11, "mov");
        m.insert(12, "arsh");
        m.insert(13, "(endian)");
        m
    };

    pub static ref REV_ALU_OPCODES: HashMap<&'static str,u8> = {
        let mut m = HashMap::new();
        m.insert("add",0);
        m.insert("sub",1);
        m.insert("mul",2);
        m.insert("div",3);
        m.insert("or",4);
        m.insert("and",5);
        m.insert("lsh",6);
        m.insert("rsh",7);
        m.insert("neg",8);
        m.insert("mod",9);
        m.insert("xor",10);
        m.insert("mov",11);
        m.insert("arsh",12);
        m.insert("(endian)",13);
        m
    };

    pub static ref JMP_OPCODES: HashMap<u8, &'static str>  = {
        let mut m = HashMap::new();
        m.insert(0, "ja");
        m.insert(1, "jeq");
        m.insert(2, "jgt");
        m.insert(3, "jge");
        m.insert(4, "jset");
        m.insert(5, "jne");
        m.insert(6, "jsgt");
        m.insert(7, "jsge");
        m.insert(8, "call");
        m.insert(9, "exit");
        m.insert(10, "jlt");
        m.insert(11, "jle");
        m.insert(12, "jslt");
        m.insert(13, "jsle");
        m
    };

    pub static ref REV_JMP_OPCODES: HashMap< &'static str,u8>  = {
        let mut m = HashMap::new();
        m.insert("ja",0);
        m.insert("jeq",1);
        m.insert("jgt",2);
        m.insert("jge",3);
        m.insert("jset",4);
        m.insert("jne",5);
        m.insert("jsgt",6);
        m.insert("jsge",7);
        m.insert("call",8);
        m.insert("exit",9);
        m.insert( "jlt",10);
        m.insert( "jle",11);
        m.insert( "jslt",12);
        m.insert( "jsle",13);
        m
    };

    pub static ref MODES: HashMap<u8, &'static str>  = {
        let mut m = HashMap::new();
        m.insert(0, "imm");
        m.insert(1, "abs");
        m.insert(2, "ind");
        m.insert(3, "mem");
        m.insert(6, "xadd");
        m
    };

    pub static ref REV_MODES: HashMap< &'static str,u8>  = {
        let mut m = HashMap::new();
        m.insert("imm", 0);
        m.insert("abs", 1);
        m.insert("ind", 2);
        m.insert("mem", 3);
        m.insert("xadd", 6);
        m
    };

    pub static ref SIZES: HashMap<u8, &'static str>  = {
        let mut m = HashMap::new();
        // word
        m.insert(0, "w");
        // half word
        m.insert(1, "h");
        // byte
        m.insert(2, "b");
        // double word
        m.insert(3, "dw");
        m
    };

    pub static ref REV_SIZES: HashMap<&'static str, u8>  = {
        let mut m = HashMap::new();
        m.insert("w", 0);
        m.insert("h", 1);
        m.insert("b", 2);
        m.insert("dw", 3);
        m
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub op: u8,
    pub regs: u8,
    pub offset: i16,
    pub imm: i32,
}

impl Instruction {
    pub fn from_bytes(bytes: &[u8]) -> Instruction {
        let mut op = 0;
        let mut regs = 0;
        let mut offset = 0;
        let mut imm = 0;
        // debug_assert!(bytes.len() == 8);

        unsafe {
            let (mut src, _): (usize, usize) = mem::transmute(bytes);
            std::ptr::copy(src as *const u8, &mut op as *mut _ as _, 1);
            src += 1;
            std::ptr::copy(src as *const u8, &mut regs as *mut _ as _, 1);
            src += 1;
            std::ptr::copy(src as *const u8, &mut offset as *mut _ as _, 2);
            src += 2;
            std::ptr::copy(src as *const u8, &mut imm as *mut _ as _, 4);
        }

        Self {
            op,
            regs,
            offset,
            imm,
        }
    }

    pub fn new(op: u8, regs: u8, offset: i16, imm: i32) -> Instruction {
        Self {
            op,
            regs,
            offset,
            imm,
        }
    }

    pub fn src_reg(&self) -> u8 {
        (self.regs >> 4) & 0xf
    }

    pub fn dst_reg(&self) -> u8 {
        self.regs & 0xf
    }

    pub fn class(&self) -> u8 {
        self.op & 0x7
    }
}
