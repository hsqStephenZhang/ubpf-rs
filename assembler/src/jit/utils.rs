use iced_x86::{Decoder, DecoderOptions, Formatter, Instruction, MasmFormatter};

#[allow(dead_code)]
pub fn display(bytes: &[u8]) {
    const HEXBYTES_COLUMN_BYTE_LENGTH: usize = 10;
    const EXAMPLE_CODE_BITNESS: u32 = 64;
    const EXAMPLE_CODE_RIP: u64 = 0x0000_7FFA_C46A_CDA4;
    let mut decoder = Decoder::with_ip(
        EXAMPLE_CODE_BITNESS,
        bytes,
        EXAMPLE_CODE_RIP,
        DecoderOptions::NONE,
    );

    let mut formatter = MasmFormatter::new();

    formatter.options_mut().set_digit_separator("`");
    formatter.options_mut().set_first_operand_char_index(10);

    let mut output = String::new();
    let mut instruction = Instruction::default();

    while decoder.can_decode() {
        decoder.decode_out(&mut instruction);
        output.clear();
        formatter.format(&instruction, &mut output);
        print!("{:016X} ", instruction.ip());
        let start_index = (instruction.ip() - EXAMPLE_CODE_RIP) as usize;
        let instr_bytes = &bytes[start_index..start_index + instruction.len()];
        for b in instr_bytes.iter() {
            print!("{:02X}", b);
        }
        if instr_bytes.len() < HEXBYTES_COLUMN_BYTE_LENGTH {
            for _ in 0..HEXBYTES_COLUMN_BYTE_LENGTH - instr_bytes.len() {
                print!("  ");
            }
        }
        println!(" {}", output);
    }
}

#[cfg(test)]
pub mod test_utils {

    use std::fs;
    use std::path::Path;

    use crate::{Instructions, integer};

    pub fn load_data(name: &str) -> (Instructions, i64) {
        let name1 = format!("../data/{}.data", name);
        let name2 = format!("../data/{}.res", name);

        let instruction_file = Path::new(&name1);
        let result_file = Path::new(&name2);

        let instructions_content = fs::read(instruction_file).unwrap();
        let instructions_content = String::from_utf8(instructions_content).unwrap();

        let result_content = fs::read(result_file).unwrap();
        let result = String::from_utf8(result_content).unwrap();
        let result = result.as_str();

        let instructions = Instructions::from_asm(&instructions_content).unwrap();

        let (_,val)=integer(result).unwrap();

        (instructions, val)
    }

    #[test]
    fn test_load_data() {
        let (instructions, res) = load_data("add");
        println!("{:?}", instructions);
        println!("{:?}", res);
    }
}
