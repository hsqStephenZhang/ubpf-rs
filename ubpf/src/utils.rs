#[cfg(test)]
pub mod test_utils {

    use std::{fs, path::Path};

    use assembler::{Instructions, integer};

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

        let (_, val) = integer(result).unwrap();

        (instructions, val)
    }

    #[test]
    fn test_load_data() {
        let (instructions, res) = load_data("add");
        println!("{:?}", instructions);
        println!("{:?}", res);
    }
}
