use crate::*;

pub fn disassemble(data: &[u8]) {
    assert_eq!(data.len() % 8, 0);

    for i in (0..data.len()).into_iter().step_by(8) {
        let r = disassemble_one(&data[i..]);
        println!("{}", r);
    }
}

pub fn disassemble_one(raw: &[u8]) -> String {
    let ins = Instruction::from_bytes(raw);
    // dbg!(ins);
    let cls = ins.class();
    let dst = ins.dst_reg();
    let src = ins.src_reg();

    if cls == class::EBPF_CLS_ALU || cls == class::EBPF_CLS_ALU64 {
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
                    return format!("{} {}, {}", opcode_name, reg(dst), imm(ins.imm),);
                } else {
                    return format!("{} {}, {}", opcode_name, reg(dst), reg(src),);
                }
            }
        }
    } else if cls == class::EBPF_CLS_JMP {
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
                        imm(src as i32),
                        offset(ins.offset)
                    );
                }
            }
        }
    } else if cls == class::EBPF_CLS_LD
        || cls == class::EBPF_CLS_LDX
        || cls == class::EBPF_CLS_ST
        || cls == class::EBPF_CLS_STX
    {
        let size = (ins.op >> 3) & 0x3; // 0x00011000
        let size_name = *SIZES.get(&size).unwrap();
        // let mode = (ins.op >> 5) & 0x7; // 0x11100000
        // let mode_name = *MODES.get(&mode).unwrap();
        let class_name = *CLASS.get(&cls).unwrap();

        // TODO: lddw
        if ins.op == 0x18 {
            let next_ins = Instruction::from_bytes(&raw[8..]);
            let double_word_imm = ((next_ins.imm as i64) << 32) + ins.imm as i64;
            return format!(
                "{}{} {}, {}",
                class_name,
                size_name,
                reg(dst),
                double_word_imm
            );
        } else if ins.op == 0x0 {
            // section instruction of lddw
            return "".into();
        }

        match cls {
            class::EBPF_CLS_LDX => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    reg(dst),
                    memory(&reg(src), ins.offset)
                );
            }
            class::EBPF_CLS_ST => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    memory(&reg(dst), ins.offset),
                    imm(ins.imm),
                );
            }
            class::EBPF_CLS_STX => {
                return format!(
                    "{}{} {}, {}",
                    class_name,
                    size_name,
                    memory(&reg(dst), ins.offset),
                    reg(src),
                );
            }
            _ => return format!("unknown instruction {}", ins.op),
        }
    } else {
        dbg!(cls);
        unreachable!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn t1() {
        let ins = [
            0xb7, 0x01, 0x00, 0x00, 0x72, 0x6c, 0x64, 0x0a, // r1 = 174353522
            0x63, 0x1a, 0xf8, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u32 *)(r10 - 8) = r1
            0x18, 0x01, 0x00, 0x00, 0x48, 0x65, 0x6c, 0x6c, 0x00, 0x00, 0x00, 0x00, 0x6f, 0x20,
            0x57, 0x6f, // r1 = 8022916924116329800 ll
            0x7b, 0x1a, 0xf0, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u64 *)(r10 - 16) = r1
            0xb7, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = 0
            0x73, 0x1a, 0xfc, 0xff, 0x00, 0x00, 0x00, 0x00, // *(u8 *)(r10 - 4) = r1
            0xbf, 0xa1, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = r10
            0x07, 0x01, 0x00, 0x00, 0xf0, 0xff, 0xff, 0xff, // r1 += -16
            0xb7, 0x02, 0x00, 0x00, 0x0d, 0x00, 0x00, 0x00, // r2 = 13
            0x85, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, // call 6
            0xb7, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r0 = 0
            0x95, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // exit
        ];
        /*
                 mov r1,#174353522
                 stxw r10-8, r1
                 lddw r1, #1819043144
                 stxdw r10-16, r1
                 mov r1,#0
                 stxb r10-4, r1
                 mov r1,r10
                 add r1,#-16
                 mov r2,#13
                 call #6
                 mov r0,#0
                 exit
        */
        let s = ins.as_slice();
        disassemble(s);
        // println!("instruction:{}", r);
    }

    #[test]
    fn t2() {
        let ins = [
            0xbf, 0xa2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r2 = r10
            0x07, 0x02, 0x00, 0x00, 0xfc, 0xff, 0xff, 0xff, // r2 += -4
            0x18, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = 0 ll
            0x85, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // call 1
            0x15, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // if r0 == 0 goto +3 <LBB0_20>
            0x79, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // r1 = *(u64 *)(r0 + 0)
            0x07, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, // r1 += 1
            0x7b, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // *(u64 *)(r0 + 0) = r1
        ];

        /*
           mov r2,r10
           add r2,#-4
           lddw r1, #4294967296
           call #1
           jeq r0,#0, 3
           ldxdw r1, r0
           add r1,#1
           stxdw r0, r1
        */

        let s = ins.as_slice();
        disassemble(s);
    }
}
