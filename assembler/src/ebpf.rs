pub mod class {
    pub const EBPF_CLS_LD: u8 = 0;
    pub const EBPF_CLS_LDX: u8 = 1;
    pub const EBPF_CLS_ST: u8 = 2;
    pub const EBPF_CLS_STX: u8 = 3;
    pub const EBPF_CLS_ALU: u8 = 4;
    pub const EBPF_CLS_JMP: u8 = 5;
    pub const EBPF_CLS_RET: u8 = 6;
    pub const EBPF_CLS_ALU64: u8 = 7;
}

pub mod alu {
    pub const NEG: u8 = 8;
    pub const END: u8 = 13;
}

pub mod op {


    use super::class::*;

    // masks

    pub const EBPF_IMM: u8 = 0x00;
    pub const EBPF_ABS: u8 = 0x20;
    pub const EBPF_IND: u8 = 0x40;
    pub const EBPF_MEM: u8 = 0x60;
    pub const EBPF_XADD: u8 = 0xc0;

    pub const EBPF_ADD: u8 = 0x00;
    pub const EBPF_SUB: u8 = 0x10;
    pub const EBPF_MUL: u8 = 0x20;
    pub const EBPF_DIV: u8 = 0x30;
    pub const EBPF_OR: u8 = 0x40;
    pub const EBPF_AND: u8 = 0x50;
    pub const EBPF_LSH: u8 = 0x60;
    pub const EBPF_RSH: u8 = 0x70;
    pub const EBPF_NEG: u8 = 0x80;
    pub const EBPF_MOD: u8 = 0x90;
    pub const EBPF_XOR: u8 = 0xa0;
    pub const EBPF_MOV: u8 = 0xb0;
    pub const EBPF_ARSH: u8 = 0xc0;
    pub const EBPF_END: u8 = 0xd0;

    pub const EBPF_JA: u8 = 0x00;
    pub const EBPF_JEQ: u8 = 0x10;
    pub const EBPF_JGT: u8 = 0x20;
    pub const EBPF_JGE: u8 = 0x30;
    pub const EBPF_JSET: u8 = 0x40;
    pub const EBPF_JNE: u8 = 0x50;
    pub const EBPF_JSGT: u8 = 0x60;
    pub const EBPF_JSGE: u8 = 0x70;
    pub const EBPF_CALL: u8 = 0x80;
    pub const EBPF_EXIT: u8 = 0x90;
    pub const EBPF_JLT: u8 = 0xa0;
    pub const EBPF_JLE: u8 = 0xb0;
    pub const EBPF_JSLT: u8 = 0xc0;
    pub const EBPF_JSLE: u8 = 0xd0;

    pub const U32_MASK: i64 = 0x00000000ffffffff;

    pub const CLS_MASK: u8 = 0x07;
    pub const ALU_OP_MASK: u8 = 0xf0;

    pub const EBPF_SRC_IMM: u8 = 0x00;
    pub const EBPF_SRC_REG: u8 = 0x08;
    pub const EBPF_SIZE_W: u8 = 0x00;
    pub const EBPF_SIZE_H: u8 = 0x08;
    pub const EBPF_SIZE_B: u8 = 0x10;
    pub const EBPF_SIZE_DW: u8 = 0x18;

    pub const EBPF_MODE_IMM: u8 = 0x00;
    pub const EBPF_MODE_MEM: u8 = 0x60;

    // real opcodes

    pub const ADD_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_ADD;
    pub const ADD_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_ADD;
    pub const SUB_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_SUB;
    pub const SUB_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_SUB;
    pub const MUL_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_MUL;
    pub const MUL_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_MUL;
    pub const DIV_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_DIV;
    pub const DIV_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_DIV;
    pub const OR_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_OR;
    pub const OR_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_OR;
    pub const AND_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_AND;
    pub const AND_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_AND;
    pub const LSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_LSH;
    pub const LSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_LSH;
    pub const RSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_RSH;
    pub const RSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_RSH;
    pub const NEG32: u8 = EBPF_CLS_ALU | EBPF_NEG;
    pub const MOD_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_MOD;
    pub const MOD_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_MOD;
    pub const XOR_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_XOR;
    pub const XOR_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_XOR;
    pub const MOV_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_MOV;
    pub const MOV_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_MOV;
    pub const ARSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_ARSH;
    pub const ARSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_ARSH;

    pub const LE: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | EBPF_END;
    pub const BE: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | EBPF_END;

    pub const ADD64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_ADD;
    pub const ADD64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_ADD;
    pub const SUB64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_SUB;
    pub const SUB64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_SUB;
    pub const MUL64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_MUL;
    pub const MUL64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_MUL;
    pub const DIV64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_DIV;
    pub const DIV64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_DIV;
    pub const OR64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_OR;
    pub const OR64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_OR;
    pub const AND64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_AND;
    pub const AND64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_AND;
    pub const LSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_LSH;
    pub const LSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_LSH;
    pub const RSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_RSH;
    pub const RSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_RSH;
    pub const NEG64: u8 = EBPF_CLS_ALU64 | EBPF_NEG;
    pub const MOD64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_MOD;
    pub const MOD64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_MOD;
    pub const XOR64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_XOR;
    pub const XOR64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_XOR;
    pub const MOV64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_MOV;
    pub const MOV64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_MOV;
    pub const ARSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | EBPF_ARSH;
    pub const ARSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | EBPF_ARSH;

    pub const LDXW: u8 = EBPF_CLS_LDX | EBPF_MODE_MEM | EBPF_SIZE_W;
    pub const LDXH: u8 = EBPF_CLS_LDX | EBPF_MODE_MEM | EBPF_SIZE_H;
    pub const LDXB: u8 = EBPF_CLS_LDX | EBPF_MODE_MEM | EBPF_SIZE_B;
    pub const LDXDW: u8 = EBPF_CLS_LDX | EBPF_MODE_MEM | EBPF_SIZE_DW;
    pub const STW: u8 = EBPF_CLS_ST | EBPF_MODE_MEM | EBPF_SIZE_W;
    pub const STH: u8 = EBPF_CLS_ST | EBPF_MODE_MEM | EBPF_SIZE_H;
    pub const STB: u8 = EBPF_CLS_ST | EBPF_MODE_MEM | EBPF_SIZE_B;
    pub const STDW: u8 = EBPF_CLS_ST | EBPF_MODE_MEM | EBPF_SIZE_DW;
    pub const STXW: u8 = EBPF_CLS_STX | EBPF_MODE_MEM | EBPF_SIZE_W;
    pub const STXH: u8 = EBPF_CLS_STX | EBPF_MODE_MEM | EBPF_SIZE_H;
    pub const STXB: u8 = EBPF_CLS_STX | EBPF_MODE_MEM | EBPF_SIZE_B;
    pub const STXDW: u8 = EBPF_CLS_STX | EBPF_MODE_MEM | EBPF_SIZE_DW;
    pub const LDDW: u8 = EBPF_CLS_LD | EBPF_MODE_IMM | EBPF_SIZE_DW;

    pub const JA: u8 = EBPF_CLS_JMP | EBPF_JA;
    pub const JEQ_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JEQ;
    pub const JEQ_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JEQ;
    pub const JGT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JGT;
    pub const JGT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JGT;
    pub const JGE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JGE;
    pub const JGE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JGE;
    pub const JSET_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JSET;
    pub const JSET_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JSET;
    pub const JNE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JNE;
    pub const JNE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JNE;
    pub const JSGT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JSGT;
    pub const JSGT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JSGT;
    pub const JSGE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JSGE;
    pub const JSGE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JSGE;

    pub const CALL: u8 = EBPF_CLS_JMP | EBPF_CALL;
    pub const EXIT: u8 = EBPF_CLS_JMP | EBPF_EXIT;

    pub const JLT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JLT;
    pub const JLT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JLT;
    pub const JLE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JLE;
    pub const JLE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JLE;
    pub const JSLT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JSLT;
    pub const JSLT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JSLT;
    pub const JSLE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | EBPF_JSLE;
    pub const JSLE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | EBPF_JSLE;
}

pub const DEFAULT_STACK_SIZE: usize = 4096;

#[test]
fn t1() {
    use self::op::*;
    let a = SUB_REG;
    let b = LDDW;
    println!("{},{}", a, b);
}
