use goblin::elf::Elf;
use goblin::elf64::sym::STT_FUNC;
use std::{collections::HashMap, ops::Range};

use crate::{
    asm_parser::{instructions, Operand},
    class,
    error::ParseError,
    op, ElfError, Instruction,
};

pub fn locate_function<'a>(elf: &Elf<'a>, target_name: &str) -> Result<Range<usize>, ElfError> {
    let idx = lookup_function(elf, target_name)?;

    let f = elf.syms.get(idx).unwrap();

    let hdr = elf.section_headers.get(f.st_shndx).unwrap();

    let offset = (hdr.sh_offset + f.st_value) as usize;
    let size = f.st_size as usize;

    return Ok(offset..(offset + size));
}

pub fn lookup_section<'a>(elf: &Elf<'a>, target_name: &str) -> Result<usize, ElfError> {
    for (index, header) in elf.section_headers.iter().enumerate() {
        let sh_name = header.sh_name;
        let name = elf.shdr_strtab.get_at(sh_name);
        println!("{:?}", name);
        match name {
            Some(r) => {
                if r == target_name {
                    return Ok(index);
                }
            }
            None => {}
        }
    }
    Err(ElfError::SectionNotFound(target_name.into()))
}

pub fn lookup_function<'a>(elf: &Elf<'a>, target_name: &str) -> Result<usize, ElfError> {
    for (index, s) in elf.syms.iter().enumerate() {
        if s.st_type() != STT_FUNC {
            continue;
        }
        let st_name = s.st_name;
        let name = elf.strtab.get_at(st_name);
        match name {
            Some(r) => {
                if r == target_name {
                    return Ok(index);
                }
            }
            None => {}
        }
    }
    Err(ElfError::FunctionNotFound(target_name.into()))
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum InstructionType {
    AluBinary,
    AluUnary,
    LoadImm,
    LoadAbs,
    LoadInd,
    LoadReg,
    StoreImm,
    StoreReg,
    JumpUnconditional,
    JumpConditional,
    Call,
    Endian(i64),
    NoOperand,
}

fn make_instruction_map() -> HashMap<String, (InstructionType, u8)> {
    let mut result = HashMap::new();

    let alu_binary_ops = &crate::REV_ALU_OPCODES;

    let mem_sizes = &crate::REV_SIZES;

    let jump_conditions = &crate::REV_JMP_OPCODES;

    {
        let mut entry = |name: &str, inst_type: InstructionType, opc: u8| {
            result.insert(name.to_string(), (inst_type, opc))
        };

        // Miscellaneous.
        entry("exit", InstructionType::NoOperand, op::EXIT);
        entry("ja", InstructionType::JumpUnconditional, op::JA);
        entry("call", InstructionType::Call, op::CALL);
        entry("lddw", InstructionType::LoadImm, op::LDDW);

        // AluUnary.
        entry("neg", InstructionType::AluUnary, op::NEG64);
        entry("neg32", InstructionType::AluUnary, op::NEG32);
        entry("neg64", InstructionType::AluUnary, op::NEG64);

        // AluBinary.
        for (&name, &opc) in alu_binary_ops.iter() {
            entry(
                name,
                InstructionType::AluBinary,
                class::EBPF_CLS_ALU64 | opc,
            );
            entry(
                &format!("{}32", name),
                InstructionType::AluBinary,
                class::EBPF_CLS_ALU | opc,
            );
            entry(
                &format!("{}64", name),
                InstructionType::AluBinary,
                class::EBPF_CLS_ALU64 | opc,
            );
        }

        // LoadAbs, LoadInd, LoadReg, StoreImm, and StoreReg.
        for (&suffix, &size) in mem_sizes.iter() {
            entry(
                &format!("ldabs{}", suffix),
                InstructionType::LoadAbs,
                op::EBPF_ABS | class::EBPF_CLS_LD | size,
            );
            entry(
                &format!("ldind{}", suffix),
                InstructionType::LoadInd,
                op::EBPF_IND | class::EBPF_CLS_LD | size,
            );
            entry(
                &format!("ldx{}", suffix),
                InstructionType::LoadReg,
                op::EBPF_MEM | class::EBPF_CLS_LDX | size,
            );
            entry(
                &format!("st{}", suffix),
                InstructionType::StoreImm,
                op::EBPF_MEM | class::EBPF_CLS_ST | size,
            );
            entry(
                &format!("stx{}", suffix),
                InstructionType::StoreReg,
                op::EBPF_MEM | class::EBPF_CLS_STX | size,
            );
        }

        // JumpConditional.
        for (&name, &condition) in jump_conditions.iter() {
            entry(
                name,
                InstructionType::JumpConditional,
                class::EBPF_CLS_JMP | condition,
            );
        }

        // Endian.
        for &size in &[16, 32, 64] {
            entry(
                &format!("be{}", size),
                InstructionType::Endian(size),
                op::BE,
            );
            entry(
                &format!("le{}", size),
                InstructionType::Endian(size),
                op::LE,
            );
        }
    }

    result
}

#[allow(warnings)]
pub fn assemble(src: &str) -> Result<Vec<Instruction>, ParseError> {
    let (_, raw_instructions) = instructions(src).map_err(|e| ParseError::ParseFailed)?;

    let mut result = Vec::with_capacity(raw_instructions.len());
    let instruction_map = make_instruction_map();

    for raw in raw_instructions {
        let name = raw.name.as_str();
        match instruction_map.get(name) {
            Some(&(inst_type, opc)) => {
                let v = &raw.operands;
                match encode(inst_type, opc, v) {
                    Ok(insn) => result.push(insn),
                    Err(msg) => todo!(),
                }
                // Special case for lddw.
                if let LoadImm = inst_type {
                    if let Some(operands) = &raw.operands {
                        if let Operand::Integer(imm) = operands[1] {
                            result.push(insn(0, 0, 0, 0, imm >> 32).unwrap());
                        }
                    }
                }
            }
            None => todo!(),
        }
    }

    // raw_instructions
    Ok(result)
}

fn operands_tuple(operands: &Option<Vec<Operand>>) -> Result<(Operand, Operand, Operand), String> {
    match operands {
        None => Ok((Operand::Nil, Operand::Nil, Operand::Nil)),
        Some(operands) => match operands.len() {
            0 => Ok((operands[0], Operand::Nil, Operand::Nil)),
            2 => Ok((operands[0], operands[1], Operand::Nil)),
            3 => Ok((operands[0], operands[1], operands[2])),
            _ => Err("Too many operands".to_string()),
        },
    }
}

#[allow(warnings)]
fn encode(
    inst_type: InstructionType,
    opc: u8,
    operands: &Option<Vec<Operand>>,
) -> Result<Instruction, ParseError> {
    let (a, b, c) = (operands_tuple(operands)).unwrap();
    match (inst_type, a, b, c) {
        (AluBinary, Operand::Register(dst), Operand::Register(src), Nil) => {
            insn(opc | op::EBPF_SRC_REG, dst, src, 0, 0)
        }
        (AluBinary, Operand::Register(dst), Operand::Integer(imm), Nil) => {
            insn(opc | op::EBPF_SRC_IMM, dst, 0, 0, imm)
        }
        (AluUnary, Operand::Register(dst), Operand::Nil, Operand::Nil) => insn(opc, dst, 0, 0, 0),
        (LoadAbs, Operand::Integer(imm), Operand::Nil, Operand::Nil) => insn(opc, 0, 0, 0, imm),
        (LoadInd, Operand::Register(src), Operand::Integer(imm), Operand::Nil) => {
            insn(opc, 0, src, 0, imm)
        }
        (LoadReg, Operand::Register(dst), Operand::Memory(src, off), Operand::Nil) => {
            insn(opc, dst, src, off, 0)
        }
        (StoreReg, Operand::Memory(dst, off), Operand::Register(src), Operand::Nil) => {
            insn(opc, dst, src, off, 0)
        }
        (StoreImm, Operand::Memory(dst, off), Operand::Integer(imm), Operand::Nil) => {
            insn(opc, dst, 0, off, imm)
        }
        (NoOperand, Operand::Nil, Operand::Nil, Operand::Nil) => insn(opc, 0, 0, 0, 0),
        (JumpUnconditional, Operand::Integer(off), Operand::Nil, Operand::Nil) => {
            insn(opc, 0, 0, off, 0)
        }
        (
            JumpConditional,
            Operand::Register(dst),
            Operand::Register(src),
            Operand::Integer(off),
        ) => insn(opc | op::EBPF_SRC_REG, dst, src, off, 0),
        (JumpConditional, Operand::Register(dst), Operand::Integer(imm), Operand::Integer(off)) => {
            insn(opc | op::EBPF_SRC_IMM, dst, 0, off, imm)
        }
        (Call, Operand::Integer(imm), Operand::Nil, Operand::Nil) => insn(opc, 0, 0, 0, imm),
        (InstructionType::Endian(size), Operand::Register(dst), Operand::Nil, Operand::Nil) => {
            insn(opc, dst, 0, 0, size)
        }
        (LoadImm, Operand::Register(dst), Operand::Integer(imm), Operand::Nil) => {
            insn(opc, dst, 0, 0, (imm << 32) >> 32)
        }
        _ => todo!(),
    }
}

fn insn(op: u8, dst: i64, src: i64, off: i64, imm: i64) -> Result<Instruction, ParseError> {
    if dst < 0 || dst >= 16 {
        return Err(ParseError::InvalidDst(dst));
    }

    if src < 0 || src >= 16 {
        return Err(ParseError::InvalidSrc(src));
    }

    if off < -32768 || off >= 32768 {
        return Err(ParseError::InvalidOffset(off));
    }

    if imm < -2147483648 || imm >= 2147483648 {
        return Err(ParseError::InvalidImmediate(imm));
    }

    let dst = dst as u8;
    let src = src as u8;
    let reg = (dst << &0xf) | (src << 4);
    let off = off as i16;
    let imm = imm as i32;
    Ok(Instruction::new(op, reg, off, imm))
}

#[cfg(test)]
mod tests {
    use crate::assemble::locate_function;
    use crate::disassemble;
    use goblin::Object;
    use std::fs;
    use std::path::Path;

    #[test]
    fn t1() {
        let path = Path::new("data/hello_kern.o");
        let buffer = fs::read(path).unwrap();
        let elf = match Object::parse(&buffer).unwrap() {
            Object::Elf(elf) => elf,
            _ => panic!(""),
        };

        let r = locate_function(&elf, "bpf_prog").unwrap();

        let ops = &buffer[r];

        disassemble(ops);
    }
}
