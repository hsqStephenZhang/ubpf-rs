use crate::{op::*, Instructions, JitBuilder};

pub const RAX: i32 = 0;
pub const RCX: i32 = 1;
pub const RDX: i32 = 2;
pub const RBX: i32 = 3;
pub const RSP: i32 = 4;
pub const RBP: i32 = 5;
pub const RSI: i32 = 6;
pub const RDI: i32 = 7;
pub const R8: i32 = 8;
pub const R9: i32 = 9;
pub const R10: i32 = 10;
pub const R11: i32 = 11;
pub const R12: i32 = 12;
pub const R13: i32 = 13;
pub const R14: i32 = 14;
pub const R15: i32 = 15;

pub const REGISTER_MAP: [i32; 11] = [RAX, RDI, RSI, RDX, R9, R8, RBX, R13, R14, R15, RBP];

// 10`0000h
pub const STACK_SIZE: usize = 4096;

pub fn translate(instructions: Instructions) -> Vec<u8> {
    let mut builder = JitBuilder::new();
    builder.emit_push(RBP);
    builder.emit_push(RBX);
    builder.emit_push(R13);
    builder.emit_push(R14);
    builder.emit_push(R15);

    if map_register(1) != RDI {
        builder.emit_mov(RDI, map_register(1));
    }

    builder.emit_mov(RSP, map_register(10));

    builder.emit_alu64_imm32(0x81, 5, RSP, 1 as i32);

    let inner = instructions.into_vec();
    for (index, ins) in inner.into_iter().enumerate() {
        // TODO: pc locations

        let dst = map_register(ins.dst_reg() as i32);
        let src = map_register(ins.src_reg() as i32);

        let target_pc = index + ins.offset as usize + 1;

        match ins.op {
            ADD_IMM => {
                builder.emit_alu32_imm32(0x81, 0, dst, ins.imm as i32);
            }
            ADD_REG => {
                builder.emit_alu32(0x01, src, dst);
            }
            SUB_IMM => {
                builder.emit_alu32_imm32(0x81, 5, dst, ins.imm as i32);
            }
            SUB_REG => {
                builder.emit_alu32(0x29, src, dst);
            }
            MUL_IMM | MUL_REG | DIV_IMM | DIV_REG | MOD_IMM | MOD_REG => {
                todo!("muldivmod")
            }
            OR_IMM => {
                builder.emit_alu32_imm32(0x81, 1, dst, ins.imm as i32);
            }
            OR_REG => {
                builder.emit_alu32(0x09, src, dst);
            }
            AND_IMM => {
                builder.emit_alu32_imm32(0x81, 4, dst, ins.imm as i32);
            }
            AND_REG => {
                builder.emit_alu32(0x21, src, dst);
            }
            LSH_IMM => {
                builder.emit_alu32_imm32(0xc1, 4, dst, ins.imm as i32);
            }
            LSH_REG => {
                builder.emit_alu32(0xd3, 4, dst);
            }
            RSH_IMM => {
                builder.emit_alu32_imm32(0xc1, 5, dst, ins.imm as i32);
            }
            RSH_REG => {
                builder.emit_alu32(0xd3, 5, dst);
            }
            NEG32 => {
                builder.emit_alu32(0xf7, 3, dst);
            }
            XOR_IMM => {
                builder.emit_alu32_imm32(0x81, 6, dst, ins.imm as i32);
            }
            XOR_REG => {
                builder.emit_alu32(0x31, src, dst);
            }
            MOV_IMM => {
                builder.emit_alu32_imm32(0xc7, 0, dst, ins.imm as i32);
            }
            MOV_REG => {
                builder.emit_mov(src, dst);
            }
            ARSH_IMM => {
                builder.emit_alu32_imm32(0xc1, 7, dst, ins.imm as i32);
            }
            ARSH_REG => {
                builder.emit_mov(src, RCX);
                builder.emit_alu32(0xd3, 7, dst);
            }
            // TODO: LE and BE
            LE => {
                todo!()
            }
            BE => {
                todo!()
            }
            ADD64_IMM => {
                builder.emit_alu64_imm32(0x81, 0, dst, ins.imm as i32);
            }
            ADD64_REG => {
                builder.emit_alu64(0x01, src, dst);
            }
            SUB64_IMM => {
                builder.emit_alu64_imm32(0x81, 5, dst, ins.imm as i32);
            }
            SUB64_REG => {
                builder.emit_alu64(0x29, src, dst);
            }
            MUL64_IMM | MUL64_REG | DIV64_IMM | DIV64_REG | MOD64_IMM | MOD64_REG => {
                todo!("muldivmod")
            }
            OR64_IMM => {
                builder.emit_alu64_imm32(0x81, 1, dst, ins.imm as i32);
            }
            OR64_REG => {
                builder.emit_alu64(0x09, src, dst);
            }
            AND64_IMM => {
                builder.emit_alu64_imm32(0x81, 4, dst, ins.imm as i32);
            }
            AND64_REG => {
                builder.emit_alu64(0x21, src, dst);
            }
            LSH64_IMM => {
                builder.emit_alu64_imm32(0xc1, 4, dst, ins.imm as i32);
            }
            LSH64_REG => {
                builder.emit_mov(src, RCX);
                builder.emit_alu64(0xd3, 4, dst);
            }
            RSH64_IMM => {
                builder.emit_alu64_imm32(0xc1, 5, dst, ins.imm as i32);
            }
            RSH64_REG => {
                builder.emit_mov(src, RCX);
                builder.emit_alu64(0xd3, 5, dst);
            }
            NEG64 => {
                builder.emit_alu64(0xf7, 3, dst);
            }
            XOR64_IMM => {
                builder.emit_alu64_imm32(0x31, 6, dst, ins.imm as i32);
            }
            XOR64_REG => {
                builder.emit_alu64(0x09, src, dst);
            }
            MOV64_IMM => {
                builder.emit_alu64_imm32(0xc7, 0, dst, ins.imm as i32);
            }
            MOV64_REG => {
                builder.emit_mov(src, dst);
            }
            ARSH64_IMM => {
                builder.emit_alu64_imm32(0xc1, 7, dst, ins.imm as i32);
            }
            ARSH64_REG => {
                builder.emit_mov(src, RCX);
                builder.emit_alu64(0xd3, 7, dst);
            }
            // load/store operations
            LDDW => {}
            LDXW => {}
            LDXH => {}
            LDXB => {}
            LDXDW => {}
            STW => {}
            STH => {}
            STB => {}
            STDW => {}
            STXW => {}
            STXH => {}
            STXB => {}
            STXDW => {}
            JA => {
                builder.emit_jmp(target_pc as i32);
            }
            JEQ_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x84, target_pc as i32);
            }
            JEQ_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x84, target_pc as i32);
            }
            JGT_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x87, target_pc as i32);
            }
            JGT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x87, target_pc as i32);
            }
            JGE_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x83, target_pc as i32);
            }
            JGE_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x83, target_pc as i32);
            }
            JLT_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x82, target_pc as i32);
            }
            JLT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x82, target_pc as i32);
            }
            JLE_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x86, target_pc as i32);
            }
            JLE_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x86, target_pc as i32);
            }
            JSET_IMM => {
                builder.emit_alu64_imm32(0xf7, 0, dst, ins.imm as i32);
                builder.emit_jcc(0x85, target_pc as i32);
            }
            JSET_REG => {
                builder.emit_alu64(0x85, src, dst);
                builder.emit_jcc(0x85, target_pc as i32);
            }
            JNE_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x85, target_pc as i32);
            }
            JNE_REG => {
                builder.emit_alu64(0x85, src, dst);
                builder.emit_jcc(0x85, target_pc as i32);
            }
            JSGT_IMM => {
                builder.emit_cmp_imm32( dst, ins.imm as i32);
                builder.emit_jcc(0x8f, target_pc as i32);
            }
            JSGT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8f, target_pc as i32);
            }
            JSGE_IMM => {
                builder.emit_cmp_imm32( dst, ins.imm as i32);
                builder.emit_jcc(0x8d, target_pc as i32);
            }
            JSGE_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8d, target_pc as i32);
            }
            JSLT_IMM => {
                builder.emit_cmp_imm32( dst, ins.imm as i32);
                builder.emit_jcc(0x8c, target_pc as i32);
            }
            JSLT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8c, target_pc as i32);
            }
            JSLE_IMM => {
                builder.emit_cmp_imm32( dst, ins.imm as i32);
                builder.emit_jcc(0x8e, target_pc as i32);
            }
            JSLE_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8e, target_pc as i32);
            }

            // TODO: call functions(maybe jit)
            CALL => {
                builder.emit_mov(R9, RCX);
                // FIXME: call extern functions
                todo!()
            }
            EXIT => {}
            _ => {}
        }
    }

    builder.emit_pop(R15);
    builder.emit_pop(R14);
    builder.emit_pop(R13);
    builder.emit_pop(RBX);
    builder.emit_pop(RBP);

    let content = &builder.buffer[..];
    content.into()
}

fn map_register(reg: i32) -> i32 {
    REGISTER_MAP[reg as usize]
}

#[cfg(test)]
mod tests {
    use super::translate;
    use crate::{jit::utils::display, Instructions};

    #[test]
    fn test_translate() {
        let instructions = Instructions::new(Vec::new());
        let r = translate(instructions);
        display(&r);
    }
}
