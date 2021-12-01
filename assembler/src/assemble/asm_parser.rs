use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, hex_digit1, space0},
    combinator::opt,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult, Parser,
};

// combine::stream::state::Stream

/// Operand of an instruction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operand {
    /// Register number.
    Register(i64),
    /// Jump offset or immediate.
    Integer(i64),
    /// Register number and offset.
    Memory(i64, i64),
    // for pattern matching
    Nil,
}

/// Parsed instruction.
#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    /// Instruction name.
    pub name: String,
    /// Operands.
    pub operands: Option<Vec<Operand>>,
}

impl Instruction {
    pub fn new(name: String, operands: Option<Vec<Operand>>) -> Instruction {
        Self { name, operands }
    }
}

pub fn ident(input: &str) -> IResult<&str, &str> {
    alphanumeric1(input)
}

pub fn integer(input: &str) -> IResult<&str, i64> {
    let sign_inner = alt((tag("+"), tag("-")));
    let sign = opt(sign_inner);

    let a = tuple((tag("0x"), hex_digit1));
    let b = tuple((tag(""), digit1));
    let number = alt((a, b));

    let mut pattern = tuple((sign, number));
    pattern(input).map(|(left, (s, (prefix, d)))| {
        let is_signed = match s {
            Some("-") => -1,
            _ => 1,
        };
        let val = if prefix.len() == 0 {
            u64::from_str_radix(d, 10).unwrap()
        } else {
            u64::from_str_radix(d, 16).unwrap()
        };
        let val = val as i64 * is_signed;
        (left, val)
    })
}

pub fn register(input: &str) -> IResult<&str, i64> {
    let mut pattern = tuple((tag("r"), digit1));
    pattern(input).map(|(next_input, (_, d))| (next_input, i64::from_str_radix(d, 10).unwrap()))
}

pub fn operand(input: &str) -> IResult<&str, Operand> {
    let register_operand = register.map(Operand::Register);
    let immediate = integer.map(Operand::Integer);
    //(u64, Option<i64>)
    let memory = delimited(tag("["), tuple((register, opt(integer))), tag("]"))
        .map(|(a, b)| Operand::Memory(a, b.unwrap_or(0)));
    let mut pattern = register_operand.or(immediate).or(memory);

    let r = pattern.parse(input);
    r
}

pub fn operands(s: &str) -> IResult<&str, Vec<Operand>> {
    separated_list1(tag(","), tuple((space0, operand)))(s)
        .map(|(next_input, s)| (next_input, s.into_iter().map(|v| v.1).collect()))
}

pub fn instruction(input: &str) -> IResult<&str, Instruction> {
    let mut pattern = tuple((space0, ident, space0, opt(operands), space0));
    // (&str, &str, &str, asm_parser::Operand, &str))
    pattern(input)
        .map(|(next_input, (_, name, _, ops, _))| (next_input, Instruction::new(name.into(), ops)))
}

pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list0(tag("\n"), instruction).parse(input)
}

#[cfg(test)]
mod tests {
    use nom::{character::complete::space0, multi::separated_list1};

    use super::*;

    #[test]
    fn test_ident() {
        let s = "mov a,1";
        let r = ident(s);
        assert_eq!(r, Ok((" a,1", "mov")));
    }

    #[test]
    fn test_digit() {
        let val = integer("0x1abcLLL").unwrap();
        println!("{:?}", val.1);
        assert_eq!(val.1, 0x1abc);
        assert_eq!(val.0, "LLL");
        let val = integer("123").unwrap();
        println!("{:?}", val.1);
        assert_eq!(val.1, 123);
    }

    #[test]
    fn test_register() {
        let val = register("r10").unwrap();
        assert_eq!(val.1, 10);
        assert_eq!(val.0, "");
    }

    #[test]
    fn test_operand() {
        let val = operand("r10").unwrap();
        assert_eq!(val.1, Operand::Register(10));

        let val = operand("123").unwrap();
        assert_eq!(val.1, Operand::Integer(123));

        let val = operand("[r10]").unwrap();
        assert_eq!(val.1, Operand::Memory(10, 0));

        let val = operand("[r10+5]").unwrap();
        assert_eq!(val.1, Operand::Memory(10, 5));

        let val = operand("[r10-5]").unwrap();
        assert_eq!(val.1, Operand::Memory(10, -5));
    }

    #[test]
    fn test_operands() {
        fn parser(s: &str) -> IResult<&str, Vec<Operand>> {
            separated_list1(tag(","), tuple((space0, operand)))(s)
                .map(|(next_input, s)| (next_input, s.into_iter().map(|v| v.1).collect()))
        }

        assert_eq!(
            parser("1, 2, 3"),
            Ok((
                "",
                vec![
                    Operand::Integer(1,),
                    Operand::Integer(2),
                    Operand::Integer(3)
                ]
            ))
        );
    }

    #[test]
    fn test_instructions() {
        // println!("{:?}", instruction.parse("mov32 r0, 0x0"),);
        // println!("{:?}",instruction.parse("mov32 r1, 2"),);
        assert_eq!(
            instruction.parse("exit"),
            Ok((
                "",
                Instruction {
                    name: "exit".to_string(),
                    operands: None,
                },
            ))
        );

        assert_eq!(
            instruction.parse("call 2"),
            Ok((
                "",
                Instruction {
                    name: "call".to_string(),
                    operands: Some(vec![Operand::Integer(2)]),
                },
            ))
        );

        assert_eq!(
            instruction.parse("addi r1, 2"),
            Ok((
                "",
                Instruction {
                    name: "addi".to_string(),
                    operands: Some(vec![Operand::Register(1), Operand::Integer(2)]),
                },
            ))
        );

        assert_eq!(
            instruction.parse("ldxb r2, [r1+2]"),
            Ok((
                "",
                Instruction {
                    name: "ldxb".to_string(),
                    operands: Some(vec![Operand::Register(2), Operand::Memory(1, 2)]),
                },
            ))
        );

        assert_eq!(
            instruction.parse("lsh r3, 0x8"),
            Ok((
                "",
                Instruction {
                    name: "lsh".to_string(),
                    operands: Some(vec![Operand::Register(3), Operand::Integer(8)]),
                },
            ))
        );

        assert_eq!(
            instruction.parse("jne r3, 0x8, +37"),
            Ok((
                "",
                Instruction {
                    name: "jne".to_string(),
                    operands: Some(vec![
                        Operand::Register(3),
                        Operand::Integer(8),
                        Operand::Integer(37)
                    ]),
                },
            ))
        );

        // Whitespace between operands is optional.
        assert_eq!(
            instruction.parse("jne r3,0x8,+37"),
            Ok((
                "",
                Instruction {
                    name: "jne".to_string(),
                    operands: Some(vec![
                        Operand::Register(3),
                        Operand::Integer(8),
                        Operand::Integer(37)
                    ]),
                },
            ))
        );
    }

    #[test]
    fn test_empty() {
        assert_eq!(instructions(""), Ok(("", vec![])));
    }

    #[test]
    fn test_exit() {
        // No operands.
        assert_eq!(
            instructions("exit"),
            Ok((
                "",
                vec![Instruction {
                    name: "exit".to_string(),
                    operands: None,
                }]
            ))
        );
    }

    #[test]
    fn test_lsh() {
        // Register and immediate operands.
        assert_eq!(
            instructions("lsh r3, 0x20"),
            Ok((
                "",
                vec![Instruction {
                    name: "lsh".to_string(),
                    operands: Some(vec![Operand::Register(3), Operand::Integer(0x20)]),
                }]
            ))
        );
    }

    #[test]
    fn test_ja() {
        // Jump offset operand.
        assert_eq!(
            instructions("ja +1"),
            Ok((
                "",
                vec![Instruction {
                    name: "ja".to_string(),
                    operands: Some(vec![Operand::Integer(1)]),
                }]
            ))
        );
    }

    #[test]
    fn test_lddw() {
        // Jump offset operand.
        assert_eq!(
            instructions("lddw r0, 0x10000000c"),
            Ok((
                "",
                vec![Instruction {
                    name: "lddw".to_string(),
                    operands: Some(vec![Operand::Register(0),Operand::Integer(0x10000000c)]),
                }]
            ))
        );
    }

    #[test]
    fn test_ldxh() {
        // Register and memory operands.
        assert_eq!(
            instructions("ldxh r4, [r1+12]"),
            Ok((
                "",
                vec![Instruction {
                    name: "ldxh".to_string(),
                    operands: Some(vec![Operand::Register(4), Operand::Memory(1, 12)]),
                }]
            ))
        );
    }

    #[test]
    fn test_tcp_sack() {
        // Sample program from ubpf.
        // We could technically indent the instructions since the parser support white spaces at
        // the beginning, but there is another test for that.
        let src = "\
ldxb r2, [r1+12]
ldxb r3, [r1+13]
lsh r3, 0x8
or r3, r2
mov r0, 0x0
jne r3, 0x8, +37
ldxb r2, [r1+23]
jne r2, 0x6, +35
ldxb r2, [r1+14]
add r1, 0xe
and r2, 0xf
lsh r2, 0x2
add r1, r2
mov r0, 0x0
ldxh r4, [r1+12]
add r1, 0x14
rsh r4, 0x2
and r4, 0x3c
mov r2, r4
add r2, 0xffffffec
mov r5, 0x15
mov r3, 0x0
jgt r5, r4, +20
mov r5, r3
lsh r5, 0x20
arsh r5, 0x20
mov r4, r1
add r4, r5
ldxb r5, [r4]
jeq r5, 0x1, +4
jeq r5, 0x0, +12
mov r6, r3
jeq r5, 0x5, +9
ja +2
add r3, 0x1
mov r6, r3
ldxb r3, [r4+1]
add r3, r6
lsh r3, 0x20
arsh r3, 0x20
jsgt r2, r3, -18
ja +1
mov r0, 0x1
exit";

        assert_eq!(
            instructions(src),
            Ok((
                "",
                vec![
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Memory(1, 12)]),
                    },
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Memory(1, 13)]),
                    },
                    Instruction {
                        name: "lsh".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Integer(8)]),
                    },
                    Instruction {
                        name: "or".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Register(2)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(0), Operand::Integer(0)]),
                    },
                    Instruction {
                        name: "jne".to_string(),
                        operands: Some(vec![
                            Operand::Register(3),
                            Operand::Integer(8),
                            Operand::Integer(37)
                        ]),
                    },
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Memory(1, 23)]),
                    },
                    Instruction {
                        name: "jne".to_string(),
                        operands: Some(vec![
                            Operand::Register(2),
                            Operand::Integer(6),
                            Operand::Integer(35)
                        ]),
                    },
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Memory(1, 14)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(1), Operand::Integer(14)]),
                    },
                    Instruction {
                        name: "and".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Integer(15)]),
                    },
                    Instruction {
                        name: "lsh".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Integer(2)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(1), Operand::Register(2)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(0), Operand::Integer(0)]),
                    },
                    Instruction {
                        name: "ldxh".to_string(),
                        operands: Some(vec![Operand::Register(4), Operand::Memory(1, 12)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(1), Operand::Integer(20)]),
                    },
                    Instruction {
                        name: "rsh".to_string(),
                        operands: Some(vec![Operand::Register(4), Operand::Integer(2)]),
                    },
                    Instruction {
                        name: "and".to_string(),
                        operands: Some(vec![Operand::Register(4), Operand::Integer(60)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Register(4)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(2), Operand::Integer(4294967276)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(5), Operand::Integer(21)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Integer(0)]),
                    },
                    Instruction {
                        name: "jgt".to_string(),
                        operands: Some(vec![
                            Operand::Register(5),
                            Operand::Register(4),
                            Operand::Integer(20)
                        ]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(5), Operand::Register(3)]),
                    },
                    Instruction {
                        name: "lsh".to_string(),
                        operands: Some(vec![Operand::Register(5), Operand::Integer(32)]),
                    },
                    Instruction {
                        name: "arsh".to_string(),
                        operands: Some(vec![Operand::Register(5), Operand::Integer(32)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(4), Operand::Register(1)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(4), Operand::Register(5)]),
                    },
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(5), Operand::Memory(4, 0)]),
                    },
                    Instruction {
                        name: "jeq".to_string(),
                        operands: Some(vec![
                            Operand::Register(5),
                            Operand::Integer(1),
                            Operand::Integer(4)
                        ]),
                    },
                    Instruction {
                        name: "jeq".to_string(),
                        operands: Some(vec![
                            Operand::Register(5),
                            Operand::Integer(0),
                            Operand::Integer(12)
                        ]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(6), Operand::Register(3)]),
                    },
                    Instruction {
                        name: "jeq".to_string(),
                        operands: Some(vec![
                            Operand::Register(5),
                            Operand::Integer(5),
                            Operand::Integer(9)
                        ]),
                    },
                    Instruction {
                        name: "ja".to_string(),
                        operands: Some(vec![Operand::Integer(2)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Integer(1)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(6), Operand::Register(3)]),
                    },
                    Instruction {
                        name: "ldxb".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Memory(4, 1)]),
                    },
                    Instruction {
                        name: "add".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Register(6)]),
                    },
                    Instruction {
                        name: "lsh".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Integer(32)]),
                    },
                    Instruction {
                        name: "arsh".to_string(),
                        operands: Some(vec![Operand::Register(3), Operand::Integer(32)]),
                    },
                    Instruction {
                        name: "jsgt".to_string(),
                        operands: Some(vec![
                            Operand::Register(2),
                            Operand::Register(3),
                            Operand::Integer(-18)
                        ]),
                    },
                    Instruction {
                        name: "ja".to_string(),
                        operands: Some(vec![Operand::Integer(1)]),
                    },
                    Instruction {
                        name: "mov".to_string(),
                        operands: Some(vec![Operand::Register(0), Operand::Integer(1)]),
                    },
                    Instruction {
                        name: "exit".to_string(),
                        operands: None,
                    }
                ]
            ))
        );
    }
}
