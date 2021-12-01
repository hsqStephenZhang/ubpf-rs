use std::collections::HashMap;

use crate::{
    class,
    error::ParseError,
    op, Instruction,
};

use super::{Operand, instructions};

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

    let alu_binary_ops = &crate::ALU_BINARY_OPS;

    let mem_sizes = &crate::SIZES;

    let jump_conditions = &crate::JMP_CONDITIONS;

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
pub(crate) fn assemble(src: &str) -> Result<Vec<Instruction>, ParseError> {
    let (_, raw_instructions) = instructions(src).map_err(|e| ParseError::ParseFailed)?;

    let mut result = Vec::with_capacity(raw_instructions.len());
    let instruction_map = make_instruction_map();

    for raw in raw_instructions {
        let name = raw.name.as_str();
        // dbg!(instruction_map.get(name), raw.clone());
        match instruction_map.get(name) {
            Some(&(inst_type, op)) => {
                let v = &raw.operands;
                match encode(inst_type, op, v) {
                    Ok(insn) => {
                        // dbg!(insn.clone());
                        result.push(insn)
                    }
                    Err(msg) => panic!("{}", msg),
                }
                // Special case for lddw.
                if let InstructionType::LoadImm = inst_type {
                    if let Some(operands) = &raw.operands {
                        if let Operand::Integer(imm) = operands[1] {
                            let insn = insn(0, 0, 0, 0, imm >> 32).unwrap();
                            result.push(insn);
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
            0 => Ok((Operand::Nil, Operand::Nil, Operand::Nil)),
            1 => Ok((operands[0], Operand::Nil, Operand::Nil)),
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
        (InstructionType::AluBinary, Operand::Register(dst), Operand::Register(src), Nil) => {
            insn(opc | op::EBPF_SRC_REG, dst, src, 0, 0)
        }
        (InstructionType::AluBinary, Operand::Register(dst), Operand::Integer(imm), Nil) => {
            insn(opc | op::EBPF_SRC_IMM, dst, 0, 0, imm)
        }
        (InstructionType::AluUnary, Operand::Register(dst), Operand::Nil, Operand::Nil) => {
            insn(opc, dst, 0, 0, 0)
        }
        (InstructionType::LoadAbs, Operand::Integer(imm), Operand::Nil, Operand::Nil) => {
            insn(opc, 0, 0, 0, imm)
        }
        (InstructionType::LoadInd, Operand::Register(src), Operand::Integer(imm), Operand::Nil) => {
            insn(opc, 0, src, 0, imm)
        }
        (
            InstructionType::LoadReg,
            Operand::Register(dst),
            Operand::Memory(src, off),
            Operand::Nil,
        ) => insn(opc, dst, src, off, 0),
        (
            InstructionType::StoreReg,
            Operand::Memory(dst, off),
            Operand::Register(src),
            Operand::Nil,
        ) => insn(opc, dst, src, off, 0),
        (
            InstructionType::StoreImm,
            Operand::Memory(dst, off),
            Operand::Integer(imm),
            Operand::Nil,
        ) => insn(opc, dst, 0, off, imm),
        (InstructionType::NoOperand, Operand::Nil, Operand::Nil, Operand::Nil) => {
            insn(opc, 0, 0, 0, 0)
        }
        (InstructionType::JumpUnconditional, Operand::Integer(off), Operand::Nil, Operand::Nil) => {
            insn(opc, 0, 0, off, 0)
        }
        (
            InstructionType::JumpConditional,
            Operand::Register(dst),
            Operand::Register(src),
            Operand::Integer(off),
        ) => insn(opc | op::EBPF_SRC_REG, dst, src, off, 0),
        (
            InstructionType::JumpConditional,
            Operand::Register(dst),
            Operand::Integer(imm),
            Operand::Integer(off),
        ) => insn(opc | op::EBPF_SRC_IMM, dst, 0, off, imm),
        (InstructionType::Call, Operand::Integer(imm), Operand::Nil, Operand::Nil) => {
            insn(opc, 0, 0, 0, imm)
        }
        (InstructionType::Endian(size), Operand::Register(dst), Operand::Nil, Operand::Nil) => {
            insn(opc, dst, 0, 0, size)
        }
        (InstructionType::LoadImm, Operand::Register(dst), Operand::Integer(imm), Operand::Nil) => {
            // println!("{:X}", imm);
            // dbg!(opc == 0x18);
            // handle lddw situation
            if opc == 0x18 {
                insn(opc, dst, 0, 0, imm)
            } else {
                insn(opc, dst, 0, 0, (imm << 32) >> 32)
            }
        }
        _ => {
            dbg!(inst_type, a, b, c);
            todo!()
        }
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

    let dst = dst as u8;
    let src = src as u8;
    let reg = (dst & 0xf) | (src << 4);
    let off = off as i16;
    Ok(Instruction::new(op, reg, off, imm))
}

#[cfg(test)]
mod tests {
    use crate::Instructions;

    #[test]
    fn t1() {
        let prog = "add64 r1, 0x605
                 mov64 r2, 0x32
                 mov64 r1, r0
                 be16 r0
                 neg64 r2
                 exit";

        println!("{:?}", Instructions::from_asm(prog).unwrap());

        println!("---------------------------");

        let buffer = [
            0x07, 0x01, 0x00, 0x00, 0x05, 0x06, 0x00, 0x00, 0xb7, 0x02, 0x00, 0x00, 0x32, 0x00,
            0x00, 0x00, 0xbf, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdc, 0x00, 0x00, 0x00,
            0x10, 0x00, 0x00, 0x00, 0x87, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x95, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let disassemble_prog = Instructions::from(buffer.as_slice());
        println!("{:?}", disassemble_prog);
    }

    #[test]
    fn t2() {
        let prog = "lddw r0, 0x10000000c";
        println!("{:?}", Instructions::from_asm(prog).unwrap());
    }
}
