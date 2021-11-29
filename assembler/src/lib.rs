mod asm_parser;
#[allow(dead_code)]
mod assemble;
mod disassemble;
mod ebpf;
mod error;
mod instruction;
pub mod utils;

use std::collections::HashMap;

// pub use assemble::*;
pub use ebpf::{alu, class, op};
pub use error::ElfError;
pub use instruction::{Instruction, Instructions};

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
