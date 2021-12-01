#[allow(dead_code)]
mod assemble;
mod ebpf;
mod error;
mod jit;
mod instruction;
pub mod utils;

use std::collections::HashMap;

// pub use assemble::*;
pub use assemble::{ident, instruction, instructions, integer, operand, operands, register};
pub use ebpf::{alu, class, op};
pub use error::ElfError;
pub use instruction::{Instruction, Instructions};
pub use jit::*;

lazy_static::lazy_static! {

    // =====> from code to name ====

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

    pub static ref ALU_OPCODES_TO_NAME: HashMap<u8, &'static str> = {
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

    pub static ref JMP_OPCODES_TO_NAME: HashMap<u8, &'static str>  = {
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

    pub static ref MODES_TO_NAME: HashMap<u8, &'static str>  = {
        let mut m = HashMap::new();
        m.insert(0, "imm");
        m.insert(1, "abs");
        m.insert(2, "ind");
        m.insert(3, "mem");
        m.insert(6, "xadd");
        m
    };


    pub static ref SIZES_TO_NAME: HashMap<u8, &'static str>  = {
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

    // =====> from name to code ====

    // binary operations
    pub static ref ALU_BINARY_OPS: HashMap<&'static str,u8> = {
        let mut m = HashMap::new();
        m.insert("add",op::EBPF_ADD);
        m.insert("sub",op::EBPF_SUB);
        m.insert("mul",op::EBPF_MUL);
        m.insert("div",op::EBPF_DIV);
        m.insert("or",op::EBPF_OR);
        m.insert("and",op::EBPF_AND);
        m.insert("lsh",op::EBPF_LSH);
        m.insert("rsh",op::EBPF_RSH);
        m.insert("mod",op::EBPF_MOD);
        m.insert("xor",op::EBPF_XOR);
        m.insert("mov",op::EBPF_MOV);
        m.insert("arsh",op::EBPF_ARSH);
        m
    };

    // conditional jmp
    pub static ref JMP_CONDITIONS: HashMap< &'static str,u8>  = {
        let mut m = HashMap::new();
        m.insert("jeq",op::EBPF_JEQ);
        m.insert("jgt",op::EBPF_JGT);
        m.insert("jge",op::EBPF_JGE);
        m.insert("jset",op::EBPF_JSET);
        m.insert("jne",op::EBPF_JNE);
        m.insert("jsgt",op::EBPF_JSGT);
        m.insert("jsge",op::EBPF_JSGE);
        m.insert( "jlt",op::EBPF_JLT);
        m.insert( "jle",op::EBPF_JLE);
        m.insert( "jslt",op::EBPF_JSLT);
        m.insert( "jsle",op::EBPF_JSLE);
        m
    };

    // modes
    pub static ref MODES: HashMap< &'static str,u8>  = {
        let mut m = HashMap::new();
        m.insert("imm", 0);
        m.insert("abs", 1);
        m.insert("ind", 2);
        m.insert("mem", 3);
        m.insert("xadd", 6);
        m
    };

    // sizes of memory access
    pub static ref SIZES: HashMap<&'static str, u8>  = {
        let mut m = HashMap::new();
        m.insert("w", op::EBPF_SIZE_W);
        m.insert("h", op::EBPF_SIZE_H);
        m.insert("b", op::EBPF_SIZE_B);
        m.insert("dw", op::EBPF_SIZE_DW);
        m
    };
}
