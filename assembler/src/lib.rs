mod asm_parser;
#[allow(dead_code)]
mod assemble;
mod consts;
mod disassemble;
mod error;
mod utils;

use std::{collections::HashMap, mem};

// pub use assemble::*;
pub use asm_parser::parse;
pub use assemble::{locate_function, lookup_function, lookup_section};
pub use consts::{alu, class, op};
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

    pub static ref MODES: HashMap<u8, &'static str>  = {
        let mut m = HashMap::new();
        m.insert(0, "imm");
        m.insert(1, "abs");
        m.insert(2, "ind");
        m.insert(3, "mem");
        m.insert(6, "xadd");
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
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub op: u8,
    pub regs: u8,
    pub offset: i16,
    pub imm: i32,
}

impl Instruction {
    pub fn new(bytes: &[u8]) -> Instruction {
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
