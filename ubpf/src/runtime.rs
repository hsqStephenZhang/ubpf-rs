use assembler::Instruction;

const MB: usize = 1024 * 1024;

const STACK_SIZE: usize = 4 * MB;
const MEM_SIZE: usize = 8 * MB;
const NUM_REGS: usize = 16;

type Regs = [i64; NUM_REGS];
type Stack = [u8; STACK_SIZE];
type Mem = [u8; MEM_SIZE];

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    instructions: Vec<Instruction>,
    pc: i64,
    memory_bound_check: bool,
    regs: Regs,
    stack: Stack,
    virtual_mem: Box<Mem>,
}

impl VirtualMachine {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            memory_bound_check: true,
            regs: [0; NUM_REGS],
            stack: [0; STACK_SIZE],
            virtual_mem: Box::new([0; MEM_SIZE]),
        }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.regs = [0; NUM_REGS];
        self.stack = [0; STACK_SIZE];

        self.regs[1] = self.virtual_mem.as_ptr() as i64;
        let stack_bottom = &self.stack as *const _ as i64;
        self.regs[10] = stack_bottom + std::mem::size_of::<Stack>() as i64;
    }

    pub fn exec(&mut self) ->Result<i64,()>{
        use assembler::op::*;
        self.reset();

        let reg = &mut self.regs;
        // let stack = &mut self.stack;

        loop {
            let cur_pc = self.pc;
            let ins = &self.instructions[cur_pc as usize];
            self.pc += 1;

            match ins.op {
                // the only instruction that advance pc
                LDDW => {
                    let new_ins = self.instructions[self.pc as usize];
                    self.pc += 1;
                    let imm_high = (new_ins.imm as i64) << 32;
                    reg[ins.dst_reg() as usize] = ins.imm as i64 | imm_high;
                }
                ADD_IMM => {
                    reg[ins.dst_reg() as usize] += ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                ADD_REG => {
                    reg[ins.dst_reg() as usize] += reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                SUB_IMM => {
                    reg[ins.dst_reg() as usize] -= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                SUB_REG => {
                    reg[ins.dst_reg() as usize] -= reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                MUL_IMM => {
                    reg[ins.dst_reg() as usize] *= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                MUL_REG => {
                    reg[ins.dst_reg() as usize] *= reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                // TODO: check divide zero
                DIV_IMM => {
                    reg[ins.dst_reg() as usize] /= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                DIV_REG => {
                    reg[ins.dst_reg() as usize] /= reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                OR_IMM => {
                    reg[ins.dst_reg() as usize] |= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                OR_REG => {
                    reg[ins.dst_reg() as usize] |= reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                AND_IMM => {
                    reg[ins.dst_reg() as usize] &= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                AND_REG => {
                    reg[ins.dst_reg() as usize] &= reg[ins.src_reg() as usize] as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                LSH_IMM => {
                    let old = reg[ins.dst_reg() as usize] & U32_MASK;
                    reg[ins.dst_reg() as usize] = old << ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                LSH_REG => {
                    let old = reg[ins.dst_reg() as usize] & U32_MASK;
                    reg[ins.dst_reg() as usize] = old << (reg[ins.src_reg() as usize] as i64);
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                RSH_IMM => {
                    let old = reg[ins.dst_reg() as usize] & U32_MASK;
                    reg[ins.dst_reg() as usize] = old >> ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                RSH_REG => {
                    let old = reg[ins.dst_reg() as usize] & U32_MASK;
                    reg[ins.dst_reg() as usize] = old >> (reg[ins.src_reg() as usize] as i64);
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                NEG => {
                    reg[ins.dst_reg() as usize] = -reg[ins.dst_reg() as usize];
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                MOD_IMM => {
                    let a = reg[ins.dst_reg() as usize] & U32_MASK;
                    let b = ins.imm as i64 & U32_MASK;
                    let r = a % b;
                    reg[ins.dst_reg() as usize] = r & U32_MASK;
                }
                MOD_REG => {
                    let a = reg[ins.dst_reg() as usize] & U32_MASK;
                    let b = reg[ins.src_reg() as usize] & U32_MASK;
                    let r = a % b;
                    reg[ins.dst_reg() as usize] = r & U32_MASK;
                }
                XOR_IMM => {
                    reg[ins.dst_reg() as usize] ^= ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                XOR_REG => {
                    reg[ins.dst_reg() as usize] ^= reg[ins.src_reg() as usize];
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                MOV_IMM => {
                    reg[ins.dst_reg() as usize] = ins.imm as i64;
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                MOV_REG => {
                    reg[ins.dst_reg() as usize] = reg[ins.src_reg() as usize];
                    reg[ins.dst_reg() as usize] &= U32_MASK;
                }
                ARSH_IMM => {
                    let a = (reg[ins.dst_reg() as usize] & U32_MASK) >> ins.imm;
                    reg[ins.dst_reg() as usize] = a & U32_MASK;
                }
                ARSH_REG => {
                    let a = (reg[ins.dst_reg() as usize] & U32_MASK) >> ins.src_reg();
                    reg[ins.dst_reg() as usize] = a & U32_MASK;
                }
                LE => {
                    todo!()
                }

                BE => {
                    todo!()
                }
                ADD64_IMM => {
                    reg[ins.dst_reg() as usize] += ins.imm as i64;
                }
                ADD64_REG => {
                    reg[ins.dst_reg() as usize] += reg[ins.src_reg() as usize];
                }
                SUB64_IMM => {
                    reg[ins.dst_reg() as usize] -= ins.imm as i64;
                }
                SUB64_REG => {
                    reg[ins.dst_reg() as usize] -= reg[ins.src_reg() as usize];
                }
                MUL64_IMM => {
                    reg[ins.dst_reg() as usize] *= ins.imm as i64;
                }
                MUL64_REG => {
                    reg[ins.dst_reg() as usize] *= reg[ins.src_reg() as usize];
                }
                DIV64_IMM => {
                    reg[ins.dst_reg() as usize] /= ins.imm as i64;
                }
                DIV64_REG => {
                    reg[ins.dst_reg() as usize] /= reg[ins.src_reg() as usize];
                }
                OR64_IMM => {
                    reg[ins.dst_reg() as usize] |= ins.imm as i64;
                }
                OR64_REG => {
                    reg[ins.dst_reg() as usize] |= reg[ins.src_reg() as usize];
                }
                AND64_IMM => {
                    reg[ins.dst_reg() as usize] &= ins.imm as i64;
                }
                AND64_REG => {
                    reg[ins.dst_reg() as usize] &= reg[ins.src_reg() as usize];
                }
                LSH64_IMM => {
                    reg[ins.dst_reg() as usize] <<= ins.imm as i64;
                }
                LSH64_REG => {
                    reg[ins.dst_reg() as usize] <<= reg[ins.src_reg() as usize];
                }
                RSH64_IMM => {
                    reg[ins.dst_reg() as usize] >>= ins.imm as i64;
                }
                RSH64_REG => {
                    reg[ins.dst_reg() as usize] >>= reg[ins.src_reg() as usize];
                }
                NEG64 => {
                    reg[ins.dst_reg() as usize] = -reg[ins.dst_reg() as usize];
                }
                MOD64_IMM => {
                    reg[ins.dst_reg() as usize] %= ins.imm as i64;
                }
                MOD64_REG => {
                    reg[ins.dst_reg() as usize] %= reg[ins.src_reg() as usize];
                }
                XOR64_IMM => {
                    reg[ins.dst_reg() as usize] ^= ins.imm as i64;
                }
                XOR64_REG => {
                    reg[ins.dst_reg() as usize] ^= reg[ins.src_reg() as usize];
                }
                MOV64_IMM => {
                    reg[ins.dst_reg() as usize] = ins.imm as i64;
                }
                MOV64_REG => {
                    reg[ins.dst_reg() as usize] = reg[ins.src_reg() as usize];
                }
                ARSH64_IMM => {
                    let old = reg[ins.dst_reg() as usize];
                    reg[ins.dst_reg() as usize] = old >> (ins.imm as i64);
                }
                ARSH64_REG => {
                    let old = reg[ins.dst_reg() as usize];
                    reg[ins.dst_reg() as usize] = old >> reg[ins.src_reg() as usize];
                }
                // load/store operations
                LDXW => {
                    let addr = reg[ins.src_reg() as usize] + ins.offset as i64;
                    reg[ins.dst_reg() as usize] = unsafe { *(addr as *const i32) as i64 };
                }
                LDXH => {
                    let addr = reg[ins.src_reg() as usize] + ins.offset as i64;
                    reg[ins.dst_reg() as usize] = unsafe { *(addr as *const i16) as i64 };
                }
                LDXB => {
                    let addr = reg[ins.src_reg() as usize] + ins.offset as i64;
                    reg[ins.dst_reg() as usize] = unsafe { *(addr as *const i8) as i64 };
                }
                LDXDW => {
                    let addr = reg[ins.src_reg() as usize] + ins.offset as i64;
                    reg[ins.dst_reg() as usize] = unsafe { *(addr as *const i64) as i64 };
                }
                STW => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i32) = ins.imm as i32 };
                }
                STH => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i16) = ins.imm as i16 };
                }
                STB => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i8) = ins.imm as i8 };
                }
                STDW => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i64) = ins.imm as i64 };
                }
                STXW => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i32) = reg[ins.src_reg() as usize] as i32 };
                }
                STXH => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i16) = reg[ins.src_reg() as usize] as i16 };
                }
                STXB => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i8) = reg[ins.src_reg() as usize] as i8 };
                }
                STXDW => {
                    let addr = reg[ins.dst_reg() as usize] + ins.offset as i64;
                    unsafe { *(addr as *mut i64) = reg[ins.src_reg() as usize] };
                }
                JA => {
                    self.pc += ins.offset as i64;
                }
                JEQ_IMM => {
                    if reg[ins.dst_reg() as usize] == ins.imm as i64 {
                        self.pc += ins.offset as i64;
                    }
                }
                JEQ_REG => {
                    if reg[ins.dst_reg() as usize] == reg[ins.src_reg() as usize] {
                        self.pc += ins.offset as i64;
                    }
                }
                JGT_IMM => {
                    if reg[ins.dst_reg() as usize] as u64 > (ins.imm as i64 | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JGT_REG => {
                    if reg[ins.dst_reg() as usize] as u64 > (reg[ins.src_reg() as usize] | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JGE_IMM => {
                    if reg[ins.dst_reg() as usize] as u64 >= (ins.imm as i64 | U32_MASK)  as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JGE_REG => {
                    if reg[ins.dst_reg() as usize] as u64 >= (reg[ins.src_reg() as usize] | U32_MASK)  as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSET_REG => {
                    if reg[ins.dst_reg() as usize] & reg[ins.src_reg() as usize] !=0 {
                        self.pc += ins.offset as i64;
                    }
                }
                JSET_IMM => {
                    if reg[ins.dst_reg() as usize] == ins.imm as i64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSGT_IMM => {
                    if reg[ins.dst_reg() as usize] > ins.imm as i64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSGT_REG => {
                    if reg[ins.dst_reg() as usize] > reg[ins.src_reg() as usize]{
                        self.pc += ins.offset as i64;
                    }
                }
                JSGE_IMM => {
                    if reg[ins.dst_reg() as usize] >= ins.imm as i64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSGE_REG => {
                    if reg[ins.dst_reg() as usize] >= reg[ins.src_reg() as usize]{
                        self.pc += ins.offset as i64;
                    }
                }
                JNE_IMM => {
                    if reg[ins.dst_reg() as usize] != ins.imm as i64 {
                        self.pc += ins.offset as i64;
                    }
                }
                JNE_REG => {
                    if reg[ins.dst_reg() as usize] != reg[ins.src_reg() as usize] {
                        self.pc += ins.offset as i64;
                    }
                }
                JLT_IMM => {
                    if (reg[ins.dst_reg() as usize] as u64) < (ins.imm as i64 | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JLT_REG => {
                    if (reg[ins.dst_reg() as usize] as u64) < (reg[ins.src_reg() as usize] | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JLE_IMM => {
                    if (reg[ins.dst_reg() as usize] as u64) <= (ins.imm as i64 | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JLE_REG => {
                    if (reg[ins.dst_reg() as usize] as u64) <= (reg[ins.src_reg() as usize] | U32_MASK) as u64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSLT_IMM => {
                    if reg[ins.dst_reg() as usize] < ins.imm as i64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSLT_REG => {
                    if reg[ins.dst_reg() as usize] < reg[ins.src_reg() as usize]{
                        self.pc += ins.offset as i64;
                    }
                }
                JSLE_IMM => {
                    if reg[ins.dst_reg() as usize] <= ins.imm as i64{
                        self.pc += ins.offset as i64;
                    }
                }
                JSLE_REG => {
                    if reg[ins.dst_reg() as usize] <= reg[ins.src_reg() as usize]{
                        self.pc += ins.offset as i64;
                    }
                }
                CALL => {
                    todo!("not implemented")
                }
                EXIT => {
                    return Ok(self.regs[0])
                }
                _ => {
                    // virtual machine show abort here
                    unreachable!()
                }
            }
        }
    }

    #[inline(always)]
    pub fn bound_check(&self) {
        if !self.memory_bound_check {
            return;
        }
    }
}

/**
 *
#define BOUNDS_CHECK_LOAD(size) \
    do { \
        if (!bounds_check(vm, (void *)reg[inst.src] + inst.offset, size, "load", cur_pc, mem, mem_len, stack)) { \
            return UINT64_MAX; \
        } \
    } while (0)

static bool
bounds_check(const struct ubpf_vm *vm, void *addr, int size, const char *type, uint16_t cur_pc, void *mem, size_t mem_len, void *stack)
{
    if (!vm->bounds_check_enabled)
        return true;
    if (mem && (addr >= mem && (addr + size) <= (mem + mem_len))) {
        /* Context access */
        return true;
    } else if (addr >= stack && (addr + size) <= (stack + STACK_SIZE)) {
        /* Stack access */
        return true;
    } else {
        fprintf(stderr, "uBPF error: out of bounds memory %s at PC %u, addr %p, size %d\n", type, cur_pc, addr, size);
        fprintf(stderr, "mem %p/%zd stack %p/%d\n", mem, mem_len, stack, STACK_SIZE);
        return false;
    }
}

 */

// pub fn compile()

#[test]
fn t1() {
    use assembler::op::U32_MASK;
    let a = U32_MASK;
    println!("{:X}", a | 1);
}
