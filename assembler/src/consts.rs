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

    pub const U32_MASK: i64 = 0x00000000ffffffff;

    pub const MASK: u8 = 0x07;
    pub const EBPF_SALU_OP_MASK: u8 = 0xf0;

    pub const EBPF_SRC_IMM: u8 = 0x00;
    pub const EBPF_SRC_REG: u8 = 0x08;
    pub const EBPF_SIZE_W: u8 = 0x00;
    pub const EBPF_SIZE_H: u8 = 0x08;
    pub const EBPF_SIZE_B: u8 = 0x10;
    pub const EBPF_SIZE_DW: u8 = 0x18;

    pub const EBPF_MODE_IMM: u8 = 0x00;
    pub const EBPF_MODE_MEM: u8 = 0x60;

    pub const ADD_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x00;
    pub const ADD_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x00;
    pub const SUB_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x10;
    pub const SUB_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x10;
    pub const MUL_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x20;
    pub const MUL_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x20;
    pub const DIV_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x30;
    pub const DIV_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x30;
    pub const OR_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x40;
    pub const OR_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x40;
    pub const AND_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x50;
    pub const AND_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x50;
    pub const LSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x60;
    pub const LSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x60;
    pub const RSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x70;
    pub const RSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x70;
    pub const NEG: u8 = EBPF_CLS_ALU | 0x80;
    pub const MOD_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0x90;
    pub const MOD_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0x90;
    pub const XOR_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0xa0;
    pub const XOR_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0xa0;
    pub const MOV_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0xb0;
    pub const MOV_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0xb0;
    pub const ARSH_IMM: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0xc0;
    pub const ARSH_REG: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0xc0;

    pub const LE: u8 = EBPF_CLS_ALU | EBPF_SRC_IMM | 0xd0;
    pub const BE: u8 = EBPF_CLS_ALU | EBPF_SRC_REG | 0xd0;

    pub const ADD64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x00;
    pub const ADD64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x00;
    pub const SUB64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x10;
    pub const SUB64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x10;
    pub const MUL64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x20;
    pub const MUL64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x20;
    pub const DIV64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x30;
    pub const DIV64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x30;
    pub const OR64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x40;
    pub const OR64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x40;
    pub const AND64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x50;
    pub const AND64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x50;
    pub const LSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x60;
    pub const LSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x60;
    pub const RSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x70;
    pub const RSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x70;
    pub const NEG64: u8 = EBPF_CLS_ALU64 | 0x80;
    pub const MOD64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0x90;
    pub const MOD64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0x90;
    pub const XOR64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0xa0;
    pub const XOR64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0xa0;
    pub const MOV64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0xb0;
    pub const MOV64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0xb0;
    pub const ARSH64_IMM: u8 = EBPF_CLS_ALU64 | EBPF_SRC_IMM | 0xc0;
    pub const ARSH64_REG: u8 = EBPF_CLS_ALU64 | EBPF_SRC_REG | 0xc0;

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
    pub const JA: u8 = EBPF_CLS_JMP | 0x00;
    pub const JEQ_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x10;
    pub const JEQ_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x10;
    pub const JGT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x20;
    pub const JGT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x20;
    pub const JGE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x30;
    pub const JGE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x30;
    pub const JSET_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x40;
    pub const JSET_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x40;
    pub const JNE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x50;
    pub const JNE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x50;
    pub const JSGT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x60;
    pub const JSGT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x60;
    pub const JSGE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0x70;
    pub const JSGE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0x70;
    pub const CALL: u8 = EBPF_CLS_JMP | 0x80;
    pub const EXIT: u8 = EBPF_CLS_JMP | 0x90;
    pub const JLT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0xa0;
    pub const JLT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0xa0;
    pub const JLE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0xb0;
    pub const JLE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0xb0;
    pub const JSLT_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0xc0;
    pub const JSLT_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0xc0;
    pub const JSLE_IMM: u8 = EBPF_CLS_JMP | EBPF_SRC_IMM | 0xd0;
    pub const JSLE_REG: u8 = EBPF_CLS_JMP | EBPF_SRC_REG | 0xd0;
}

#[test]
fn t1() {
    use self::op::*;
    let a = SUB_REG;
    let b = LDDW;
    println!("{},{}", a, b);
}
