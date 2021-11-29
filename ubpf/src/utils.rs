#[cfg(test)]
pub mod test_utils {

    use assembler::Instructions;
    use assembler::integer;
    use std::fs;
    use std::path::Path;

    pub fn load_data(name: &str) -> (Instructions, i64) {
        let name1 = format!("data/{}.data", name);
        let name2 = format!("data/{}.res", name);

        let instruction_file = Path::new(&name1);
        let result_file = Path::new(&name2);

        let instructions_content = fs::read(instruction_file).unwrap();
        let instructions_content = String::from_utf8(instructions_content).unwrap();

        // println!("{}", instructions_content);

        let result_content = fs::read(result_file).unwrap();
        let result = String::from_utf8(result_content).unwrap();
        let result = result.as_str();

        let a = Instructions::from_asm(&instructions_content).unwrap();

        // if result.starts_with("0x"){
        //     return i64::
        // }

        // let sig = result.chars().nth(0);
        // let iter=result.chars().into_iter();

        // let s = match result_content[0] {
        //     Some('-') => -1,
        //     Some('+') => 1,
        //     Some(_) => unreachable!(),
        //     None => 1,
        // };

        let (_,val)=integer(result).unwrap();

        (a, val)
    }

    #[test]
    fn test_load_data() {
        let (instructions, res) = load_data("add");
        println!("{:?}", instructions);
        println!("{:?}", res);
    }
}
