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
            MUL_IMM => {}
            MUL_REG => {}
            DIV_IMM => {}
            DIV_REG => {}
            OR_IMM => {
                builder.emit_alu32_imm32(0x81, 1, dst, ins.imm as i32);
            }
            OR_REG => {}
            AND_IMM => {
                builder.emit_alu32_imm32(0x81, 4, dst, ins.imm as i32);
            }
            AND_REG => {}
            LSH_IMM => {
                builder.emit_alu32_imm32(0xc1, 4, dst, ins.imm as i32);
            }
            LSH_REG => {}
            RSH_IMM => {
                builder.emit_alu32_imm32(0xc1, 5, dst, ins.imm as i32);
            }
            RSH_REG => {}
            NEG32 => {}
            MOD_IMM => {}
            MOD_REG => {}
            XOR_IMM => {}
            XOR_REG => {}
            MOV_IMM => {}
            MOV_REG => {}
            ARSH_IMM => {}
            ARSH_REG => {}
            // TODO: LE and BE
            LE => {
                todo!()
            }
            BE => {
                todo!()
            }
            ADD64_IMM => {}
            ADD64_REG => {}
            SUB64_IMM => {}
            SUB64_REG => {}
            MUL64_IMM => {}
            MUL64_REG => {}
            DIV64_IMM => {}
            DIV64_REG => {}
            OR64_IMM => {}
            OR64_REG => {}
            AND64_IMM => {}
            AND64_REG => {}
            LSH64_IMM => {}
            LSH64_REG => {}
            RSH64_IMM => {}
            RSH64_REG => {}
            NEG64 => {}
            MOD64_IMM => {}
            MOD64_REG => {}
            XOR64_IMM => {}
            XOR64_REG => {}
            MOV64_IMM => {}
            MOV64_REG => {}
            ARSH64_IMM => {}
            ARSH64_REG => {}
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
            JA => {}
            JEQ_IMM => {}
            JEQ_REG => {}
            JGT_IMM => {}
            JGT_REG => {}
            JGE_IMM => {}
            JGE_REG => {}
            JSET_REG => {}
            JSET_IMM => {}
            JSGT_IMM => {}
            JSGT_REG => {}
            JSGE_IMM => {}
            JSGE_REG => {}
            JNE_IMM => {}
            JNE_REG => {}
            JLT_IMM => {}
            JLT_REG => {}
            JLE_IMM => {}
            JLE_REG => {}
            JSLT_IMM => {}
            JSLT_REG => {}
            JSLE_IMM => {}
            JSLE_REG => {}
            // TODO: call functions(maybe jit)
            CALL => {}
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
