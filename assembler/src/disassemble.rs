use crate::*;

pub fn disassemble(data: &[u8]) {
    assert_eq!(data.len() % 8, 0);

    for i in (0..data.len()).into_iter().step_by(8) {
        let r = disassemble_one(&data[i..i + 8]);
        println!("{}", r);
    }
}

pub fn disassemble_one(raw: &[u8]) -> String {
    let ins = Instruction::new(raw);
    let cls = ins.class();
    let dst=ins.dst_reg();
    let src=ins.src_reg();

    if cls == class::ALU || cls == class::ALU64 {
        let source = (ins.op >> 3) & 0x1; // 0x00001000
        let opcode = (ins.op >> 4) & 0xf; // 0x11110000
        let opcode_name = *ALU_OPCODES.get(&opcode).unwrap();

        match opcode {
            alu::END => {
                let opcode_name = if source == 1 { "be" } else { "le" };
                return format!("{}{} {}", opcode_name, ins.imm, reg(ins.dst_reg()));
            }
            alu::NEG => {
                return format!("{} {}", opcode_name, reg(dst));
            }
            _ => {
                if source == 0 {
                    return format!("{} {},{}", opcode_name, reg(dst), imm(ins.imm),);
                } else {
                    return format!(
                        "{} {},{}",
                        opcode_name,
                        reg(dst),
                        reg(src),
                    );
                }
            }
        }
    } else if cls == class::JMP {
        let source = (ins.op >> 3) & 0x1; // 0x00001000
        let opcode = (ins.op >> 4) & 0xf; // 0x11110000
        let opcode_name = *JMP_OPCODES.get(&opcode).unwrap();
        match opcode_name {
            "exit" => return opcode_name.to_owned(),
            "call" => return format!("{} {}", opcode_name, imm(ins.imm)),
            "ja" => return format!("{} {}", opcode_name, offset(ins.offset)),
            _ => {
                if source == 0 {
                    return format!(
                        "{} {},{}, {}",
                        opcode_name,
                        reg(dst),
                        imm(ins.imm),
                        offset(ins.offset)
                    );
                } else {
                    return format!(
                        "{} {},{}, {}",
                        opcode_name,
                        reg(dst),
                        imm(src as u32),
                        offset(ins.offset)
                    );
                }
            }
        }
    } else if cls == class::LD || cls == class::LDX || cls == class::ST || cls == class::STX {
        let size = (ins.op >> 3) & 0x3; // 0x00011000
        let size_name = *SIZES.get(&size).unwrap();
        // let mode = (ins.op >> 5) & 0x7; // 0x11100000
        // let mode_name = *MODES.get(&mode).unwrap();

        // TODO: lddw
        if ins.op == 0x18 {
        } else if ins.op == 0x0 {
            // section instruction of lddw
            return "".into();
        }

        let class_name = *CLASS.get(&cls).unwrap();

        match cls {
            class::LDX => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    reg(dst),
                    memory(&reg(src), ins.offset)
                );
            }
            class::ST => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    memory(&reg(dst), ins.offset),
                    imm(ins.imm),
                );
            }
            class::STX => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    memory(&reg(dst), ins.offset),
                    reg(src),
                );
            }
            _ => return format!("unkown instruction {}", ins.op),
        }
    } else {
        unreachable!()
    }
}
