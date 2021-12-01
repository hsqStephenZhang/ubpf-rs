use crate::{class::EBPF_CLS_ALU64, ebpf::DEFAULT_STACK_SIZE, op::*, Instruction, JitBuilder};

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

const TARGET_PC_EXIT: i32 = -1;
const TARGET_PC_DIV_BY_ZERO: i32 = -2;

pub fn translate(inner: &Vec<Instruction>) -> Vec<u8> {
    let mut builder = JitBuilder::new();

    // save stack frame
    builder.emit_push(RBP);
    builder.emit_mov(RSP, map_register(10));
    builder.emit_alu64_imm32(0x81, 5, RSP, DEFAULT_STACK_SIZE as i32);

    // save registers
    builder.emit_push(RBX);
    builder.emit_push(R13);
    builder.emit_push(R14);
    builder.emit_push(R15);

    if map_register(1) != RDI {
        builder.emit_mov(RDI, map_register(1));
    }

    let num_ins = inner.len();
    for (index, ins) in inner.iter().enumerate() {
        // TODO: pc locations
        builder.pc_locations.push(builder.offset);

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
                muldivmod(
                    &mut builder,
                    ins.op,
                    src,
                    dst,
                    ins.imm as i32,
                    target_pc as i64,
                );
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
                builder.emit_alu32(0xc1, 4, dst);
                builder.emit1(ins.imm as u8);
            }
            LSH_REG => {
                builder.emit_alu32(0xd3, 4, dst);
            }
            RSH_IMM => {
                builder.emit_alu32(0xc1, 5, dst);
                builder.emit1(ins.imm as u8);
                // builder.emit_alu32_imm32(0xc1, 5, dst, ins.imm as i32);
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
                muldivmod(
                    &mut builder,
                    ins.op,
                    src,
                    dst,
                    ins.imm as i32,
                    target_pc as i64,
                );
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
                builder.emit_alu64(0xc1, 4, dst);
                builder.emit1(ins.imm as u8);
                // builder.emit_alu64_imm32(ins.imm as i32);
            }
            LSH64_REG => {
                builder.emit_mov(src, RCX);
                builder.emit_alu64(0xd3, 4, dst);
            }
            RSH64_IMM => {
                builder.emit_alu64(0xc1, 5, dst);
                builder.emit1(ins.imm as u8);
                // builder.emit_alu64_imm32(0xc1, 5, dst, ins.imm as i32);
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
            LDDW => {
                builder.emit_load_imm(dst, ins.imm);
            }
            LDXW => {
                builder.emit_load(crate::OperandSize::S32, src, dst, ins.offset as i32);
            }
            LDXH => {
                builder.emit_load(crate::OperandSize::S16, src, dst, ins.offset as i32);
            }
            LDXB => {
                builder.emit_load(crate::OperandSize::S8, src, dst, ins.offset as i32);
            }
            LDXDW => {
                builder.emit_load(crate::OperandSize::S64, src, dst, ins.offset as i32);
            }
            STW => {
                builder.emit_store_imm32(
                    crate::OperandSize::S32,
                    dst,
                    ins.offset as i32,
                    ins.imm as i32,
                );
            }
            STH => {
                builder.emit_store_imm32(
                    crate::OperandSize::S16,
                    dst,
                    ins.offset as i32,
                    ins.imm as i32,
                );
            }
            STB => {
                builder.emit_store_imm32(
                    crate::OperandSize::S8,
                    dst,
                    ins.offset as i32,
                    ins.imm as i32,
                );
            }
            STDW => {
                builder.emit_store_imm32(
                    crate::OperandSize::S64,
                    dst,
                    ins.offset as i32,
                    ins.imm as i32,
                );
            }
            STXW => {
                builder.emit_store(crate::OperandSize::S32, src, dst, ins.offset as i32);
            }
            STXH => {
                builder.emit_store(crate::OperandSize::S16, src, dst, ins.offset as i32);
            }
            STXB => {
                builder.emit_store(crate::OperandSize::S8, src, dst, ins.offset as i32);
            }
            STXDW => {
                builder.emit_store(crate::OperandSize::S64, src, dst, ins.offset as i32);
            }
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
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x8f, target_pc as i32);
            }
            JSGT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8f, target_pc as i32);
            }
            JSGE_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x8d, target_pc as i32);
            }
            JSGE_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8d, target_pc as i32);
            }
            JSLT_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
                builder.emit_jcc(0x8c, target_pc as i32);
            }
            JSLT_REG => {
                builder.emit_cmp(src, dst);
                builder.emit_jcc(0x8c, target_pc as i32);
            }
            JSLE_IMM => {
                builder.emit_cmp_imm32(dst, ins.imm as i32);
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
            EXIT => {
                if index != num_ins - 1 {
                    builder.emit_jmp(TARGET_PC_EXIT);
                }
            }
            _ => {}
        }
    }

    builder.exit_location = builder.offset;

    builder.emit_pop(R15);
    builder.emit_pop(R14);
    builder.emit_pop(R13);
    builder.emit_pop(RBX);
    builder.emit1(0xc9);
    builder.emit1(0xc3); /* ret */

    let content = &builder.buffer[..];
    let mut content: Vec<u8> = content.into();

    dbg!(builder.jumps.clone());
    dbg!(builder.pc_locations.clone());

    for jump in builder.jumps.iter() {
        let target_location = if jump.target_pc == TARGET_PC_EXIT {
            builder.exit_location
        } else if jump.target_pc == TARGET_PC_DIV_BY_ZERO {
            unimplemented!("todo: div by zero check");
        } else {
            builder.pc_locations[jump.target_pc as usize]
        };

        dbg!(target_location, jump);

        let relative = target_location - jump.offset_location - std::mem::size_of::<i32>();

        unsafe {
            let p = content.as_mut_ptr();
            let offset_ptr = (p as usize + jump.offset_location) as *mut i32;
            let content: &[u8] = std::mem::transmute((offset_ptr, 10));
            println!("{:X?}", content);
            *offset_ptr = relative as i32;
            println!("{:X?}", content);
        }
    }

    content
}

fn map_register(reg: i32) -> i32 {
    REGISTER_MAP[reg as usize]
}

#[allow(unused_variables)]
fn muldivmod(builder: &mut JitBuilder, opcode: u8, src: i32, dst: i32, imm: i32, pc: i64) {
    // MUL_IMM | MUL_REG | DIV_IMM | DIV_REG | MOD_IMM | MOD_REG
    let mul_res = (opcode & ALU_OP_MASK) == (MUL_IMM & ALU_OP_MASK);
    let div_res = (opcode & ALU_OP_MASK) == (DIV_IMM & ALU_OP_MASK);
    let mod_res = (opcode & ALU_OP_MASK) == (MOD_IMM & ALU_OP_MASK);
    let is64 = (opcode & CLS_MASK) == EBPF_CLS_ALU64;

    // TODO: divide by zero check
    // if div_res || mod_res {
    //     builder.emit_load_imm(RCX, pc);

    //     /* test src,src */
    //     if is64 {
    //         builder.emit_alu64(0x85, src, src);
    //     } else {
    //         builder.emit_alu32(0x85, src, src);
    //     }

    //     /* jz div_by_zero */
    //     builder.emit_jcc(0x84, TARGET_PC_DIV_BY_ZERO);
    // }

    if dst != RAX {
        builder.emit_push(RAX);
    }
    if dst != RDX {
        builder.emit_push(RDX);
    }
    if imm != 0 {
        builder.emit_load_imm(RCX, imm as i64);
    } else {
        builder.emit_mov(src, RCX);
    }

    builder.emit_mov(dst, RAX);

    if div_res || mod_res {
        /* xor %edx,%edx */
        builder.emit_alu32(0x31, RDX, RDX);
    }

    if is64 {
        builder.emit_rex(1, 0, 0, 0);
    }

    /* mul %ecx or div %ecx */
    builder.emit_alu32(0xf7, if mul_res { 4 } else { 6 }, RCX);

    if dst != RDX {
        if mod_res {
            builder.emit_mov(RDX, dst);
        }
        builder.emit_pop(RDX);
    }
    if dst != RAX {
        if div_res || mul_res {
            builder.emit_mov(RAX, dst);
        }
        builder.emit_pop(RAX);
    }
}

#[allow(unused_variables, dead_code)]
fn muldivmod_nop(builder: &mut JitBuilder, opcode: u8, src: i32, dst: i32, imm: i32, pc: i64) {}

#[cfg(test)]
mod tests {
    use super::translate;
    use crate::jit::utils::{display, test_utils::load_data};

    #[allow(dead_code)]
    fn test_translate(prog_name: &str) {
        let (instructions, res) = load_data(prog_name);
        let r = translate(&instructions.into());
        display(&r);
        println!("----\nres:{:?}\n\n", res);
    }

    use libc::*;
    use nom::AsBytes;

    const PAGE_SIZE: usize = 4096;

    pub fn page_align(n: usize) -> usize {
        return (n + (PAGE_SIZE - 1)) & !(PAGE_SIZE - 1);
    }

    fn test_suite(prog_name: &str, memory: (*const u8, usize)) {
        let (instructions, res) = load_data(prog_name);
        let r = translate(&instructions.into());
        display(&r);
        let size = page_align(r.len());
        unsafe {
            let fn_base = mmap(
                0 as _,
                size,
                PROT_READ | PROT_WRITE | PROT_EXEC,
                MAP_PRIVATE | MAP_ANONYMOUS,
                -1,
                0,
            );

            let (src, slen) = std::mem::transmute(r.as_bytes());
            std::ptr::copy(src, fn_base, slen);

            let f: fn(*const u8, usize) -> i64 = std::mem::transmute(fn_base);
            let r = f(memory.0, memory.1);
            assert_eq!(r, res);

            munmap(fn_base, size);
        }
    }

    fn null_mem() -> (*const u8, usize) {
        (0 as _, 0)
    }

    #[test]
    fn test_add() {
        test_suite("add", null_mem());
    }

    #[test]
    fn test_div32() {
        test_suite("div32_imm", null_mem());
        test_suite("div32_reg", null_mem());
    }

    #[test]
    fn test_div64() {
        test_suite("div64_imm", null_mem());
        test_suite("div64_reg", null_mem());
    }

    #[test]
    fn test_jmp() {
        test_suite("ja", null_mem());

        test_suite("jeq_imm", null_mem());
        test_suite("jeq_reg", null_mem());

        test_suite("jge_imm", null_mem());
        test_suite("jgt_imm", null_mem());

        test_suite("jgt_reg", null_mem());

        test_suite("jle_imm", null_mem());

        test_suite("jset_reg", null_mem());

        test_suite("jsge_reg", null_mem());
        test_suite("jsge_imm", null_mem());

        test_suite("jsgt_reg", null_mem());
        test_suite("jsgt_imm", null_mem());

        test_suite("jsle_reg", null_mem());
        test_suite("jsle_imm", null_mem());

        test_suite("jslt_reg", null_mem());
        test_suite("jslt_imm", null_mem());
    }

    #[test]
    fn test_memory() {
        let raw: [u8; 8] = [0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0xcc, 0xdd];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("ldxw", mem);

        let raw: [u8; 8] = [0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0xcc, 0xdd];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("ldxh", mem);

        let raw: [u8; 8] = [0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0xcc, 0xdd];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("ldxb", mem);

        let raw: [u8; 12] = [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd,
        ];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("ldxdw", mem);
    }
    
    #[test]
    fn test_ldx() {
        let raw: [u8; 12] = [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd,
        ];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("stxw", mem);

        let raw: [u8; 12] = [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd,
        ];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("stxh", mem);

        let raw: [u8; 12] = [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd,
        ];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("stxb", mem);

        let raw: [u8; 12] = [
            0xaa, 0xbb, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0xcc, 0xdd,
        ];
        let mem = unsafe { std::mem::transmute(raw.as_slice()) };
        test_suite("stxdw", mem);
    }
}
