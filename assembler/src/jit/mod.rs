use bytes::{BufMut, BytesMut};

pub mod translate;
pub mod utils;

pub use translate::{R10, R11, R12, R13, R14, R15, R8, R9, RBP, RBX, RCX, RDI, RDX, RSI, RSP};

use self::translate::RAX;

#[derive(Debug, Clone)]
pub enum OperandSize {
    S8,
    S16,
    S32,
    S64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Jmp {
    offset_location: usize,
    target_pc: i32,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct JitBuilder {
    pub buffer: BytesMut,
    pc_locations: Vec<usize>,
    exit_location: usize,
    jumps: Vec<Jmp>,
    num_jmp: usize,
    offset: usize,
}

// TODO: big endian CS small endian
impl JitBuilder {
    pub fn new() -> JitBuilder {
        Self {
            buffer: BytesMut::new(),
            pc_locations: Vec::new(),
            exit_location: 0,
            jumps: Vec::new(),
            num_jmp: 0,
            offset: 0,
        }
    }

    #[inline(always)]
    pub fn emit1(&mut self, val: u8) {
        self.buffer.put_u8(val);
    }

    #[inline(always)]
    pub fn emit2(&mut self, val: u16) {
        let buffer: [u8; 2] = unsafe { std::mem::transmute(val) };
        self.buffer.put_u8(buffer[0]);
        self.buffer.put_u8(buffer[1]);
    }

    #[inline(always)]
    pub fn emit4(&mut self, val: u32) {
        let buffer: [u8; 4] = unsafe { std::mem::transmute(val) };
        // if buffer[3] == 0 && buffer[2] == 0 && buffer[1] == 0 {
        //     self.buffer.put_u8(buffer[0]);
        //     return;
        // }
        self.buffer.put_u8(buffer[0]);
        self.buffer.put_u8(buffer[1]);
        self.buffer.put_u8(buffer[2]);
        self.buffer.put_u8(buffer[3]);
    }

    #[inline(always)]
    pub fn emit8(&mut self, val: u64) {
        let buffer: [u8; 8] = unsafe { std::mem::transmute(val) };
        self.buffer.put_u8(buffer[0]);
        self.buffer.put_u8(buffer[1]);
        self.buffer.put_u8(buffer[2]);
        self.buffer.put_u8(buffer[3]);
        self.buffer.put_u8(buffer[4]);
        self.buffer.put_u8(buffer[5]);
        self.buffer.put_u8(buffer[6]);
        self.buffer.put_u8(buffer[7]);
    }

    #[allow(unused_variables)]
    #[inline(always)]
    pub fn emit_jump_offset(&mut self, target_pc: i32) {
        // let jmp = &mut self.jumps[self.num_jmp];
        // self.num_jmp += 1;
        // jmp.offset_location = self.offset;
        // jmp.target_pc = target_pc;
        self.emit4(0); // jmp place holder
    }

    #[inline(always)]
    pub fn emit_rex(&mut self, w: i32, r: i32, x: i32, b: i32) {
        assert!(!(w & !1) != 0);
        assert!(!(r & !1) != 0);
        assert!(!(x & !1) != 0);
        assert!(!(b & !1) != 0);
        let val = (0x40 | (w << 3) | (r << 2) | (x << 1) | b) as u8;
        // dbg!(w, r, x, b);
        self.emit1(val);
    }

    #[inline(always)]
    pub fn emit_basic_rex(&mut self, w: i32, src: i32, dst: i32) {
        if w != 0 || (src & 8) != 0 || (dst & 8) != 0 {
            self.emit_rex(w, bit(src & 8), 0, bit(dst & 8));
        }
    }

    #[inline(always)]
    pub fn emit_push(&mut self, r: i32) {
        self.emit_basic_rex(0, 0, r); // w=0, src=0, dst=r
                                      // 0x5X, X is the register
        self.emit1(0x50 | (r & 7) as u8);
    }

    #[inline(always)]
    pub fn emit_pop(&mut self, r: i32) {
        self.emit_basic_rex(0, 0, r);
        self.emit1(0x58 | (r & 7) as u8);
    }

    // 0x11 | 111 | 111 -> mode | r | m
    #[inline(always)]
    pub fn emit_modrm(&mut self, mode: i32, r: i32, m: i32) {
        self.emit1(((mode & 0xc0) | ((r & 7) << 3) | (m & 7)) as u8);
    }

    #[inline(always)]
    pub fn emit_modrm_reg2reg(&mut self, r: i32, m: i32) {
        self.emit_modrm(0xc0, r, m);
    }

    #[inline(always)]
    pub fn emit_alu32(&mut self, op: i32, src: i32, dst: i32) {
        self.emit_basic_rex(0, src, dst);
        self.emit1(op as u8);
        self.emit_modrm_reg2reg(src, dst);
    }

    #[inline(always)]
    pub fn emit_alu32_imm32(&mut self, op: i32, src: i32, dst: i32, imm: i32) {
        self.emit_alu32(op, src, dst);
        self.emit4(imm as u32);
    }

    #[inline(always)]
    pub fn emit_alu32_imm8(&mut self, op: i32, src: i32, dst: i32, imm: i8) {
        self.emit_alu32(op, src, dst);
        self.emit1(imm as u8);
    }

    #[inline(always)]
    pub fn emit_alu64(&mut self, op: i32, src: i32, dst: i32) {
        self.emit_basic_rex(1, src, dst);
        self.emit1(op as u8);
        self.emit_modrm_reg2reg(src, dst);
    }

    #[inline(always)]
    pub fn emit_alu64_imm32(&mut self, op: i32, src: i32, dst: i32, imm: i32) {
        self.emit_alu64(op, src, dst);
        self.emit4(imm as u32);
    }

    #[inline(always)]
    pub fn emit_alu64_imm8(&mut self, src: i32, dst: i32) {
        self.emit_alu64(0x89, src, dst);
    }

    #[inline(always)]
    pub fn emit_mov(&mut self, src: i32, dst: i32) {
        self.emit_alu64(0x89, src, dst);
    }

    #[inline(always)]
    pub fn emit_cmp_imm32(&mut self, dst: i32, imm: i32) {
        self.emit_alu64_imm32(0x81, 7, dst, imm);
    }

    #[inline(always)]
    pub fn emit_cmp(&mut self, src: i32, dst: i32) {
        self.emit_alu64(0x39, src, dst);
    }

    #[inline(always)]
    pub fn emit_jcc(&mut self, code: i32, target_pc: i32) {
        self.emit1(0x0f);
        self.emit1(code as u8);
        self.emit_jump_offset(target_pc);
    }

    #[inline(always)]
    pub fn emit_load(&mut self, size: OperandSize, src: i32, dst: i32, offset: i32) {
        let a = match size {
            OperandSize::S64 => 1,
            _ => 0,
        };
        self.emit_basic_rex(a, dst, src);

        match size {
            OperandSize::S8 => {
                self.emit1(0x0f);
                self.emit1(0xb6);
            }
            OperandSize::S16 => {
                self.emit1(0x0f);
                self.emit1(0xb7);
            }
            OperandSize::S32 | OperandSize::S64 => {
                self.emit1(0x8b);
            }
        }

        self.emit_modrm_and_displacement(dst, src, offset);
    }

    #[inline(always)]
    pub fn emit_load_imm(&mut self, dst: i32, imm: i64) {
        // dbg!(imm >= i32::MIN as i64 && imm >= i32::MAX as i64);
        if imm >= i32::MIN as i64 && imm >= i32::MAX as i64 {
            self.emit_alu64_imm32(0xc7, 0, dst, imm as i32);
        } else {
            self.emit_basic_rex(1, 0, dst);
            self.emit1(0xb8 | (dst & 7) as u8);
            self.emit8(imm as u64);
        }
    }

    #[inline(always)]
    pub fn emit_store(&mut self, size: OperandSize, src: i32, dst: i32, offset: i32) {
        match size {
            OperandSize::S16 => {
                self.emit1(0x66);
            }
            _ => {}
        }
        let rexw = match size {
            OperandSize::S64 => 1,
            _ => 0,
        };

        let a = match size {
            OperandSize::S8 => 1,
            _ => 0,
        };

        if (rexw | src & 8 | dst & 8 | a) != 0 {
            self.emit_rex(rexw, bit(src & 8), 0, bit(dst & 8));
        }

        let b = match size {
            OperandSize::S8 => 0x88,
            _ => 0x89,
        };
        self.emit1(b);
        self.emit_modrm_and_displacement(src, dst, offset);
    }

    #[inline(always)]
    pub fn emit_store_imm32(&mut self, size: OperandSize, dst: i32, offset: i32, imm: i32) {
        if let OperandSize::S16 = size {
            self.emit1(0x66);
        }
        let a = match size {
            OperandSize::S64 => 0,
            _ => 1,
        };
        self.emit_basic_rex(a, 0, dst);
        let b = match size {
            OperandSize::S8 => 0xc6,
            _ => 0xc7,
        };
        self.emit1(b);
        self.emit_modrm_and_displacement(0, dst, offset);
        match size {
            OperandSize::S32 | OperandSize::S64 => {
                self.emit4(imm as u32);
            }
            OperandSize::S16 => {
                self.emit2(imm as u16);
            }
            OperandSize::S8 => {
                self.emit1(imm as u8);
            }
        }
    }

    #[inline(always)]
    pub fn emit_modrm_and_displacement(&mut self, r: i32, m: i32, d: i32) {
        if d == 0 && (m & 7) != RBP {
            self.emit_modrm(0x00, r, m);
        } else if d >= -128 && d <= 127 {
            self.emit_modrm(0x40, r, m);
            self.emit1(d as u8);
        } else {
            self.emit_modrm(0x80, r, m);
            self.emit4(d as u32);
        }
    }

    #[inline(always)]
    pub fn emit_call(&mut self, target: *const u8) {
        let addr = target as i64;
        self.emit_load_imm(RAX, addr);
        self.emit1(0xff);
        self.emit1(0xd0);
    }

    #[inline(always)]
    pub fn emit_jmp(&mut self, target_pc: i32) {
        self.emit1(0xe9);
        self.emit_jump_offset(target_pc);
    }
}

#[inline(always)]
fn bit(val: i32) -> i32 {
    if val & 8 == 0 {
        0
    } else {
        1
    }
}
