use std::ops::Range;

use goblin::{elf::Elf, elf64::sym::STT_FUNC};

use crate::ElfError;

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
        if let Some(r) = name
            && r == target_name
        {
            return Ok(index);
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
        if let Some(r) = name
            && r == target_name
        {
            return Ok(index);
        }
    }
    Err(ElfError::FunctionNotFound(target_name.into()))
}

#[cfg(test)]
mod tests {
    use std::{fs, path::Path};

    use goblin::Object;

    use crate::{Instructions, assemble::elf::locate_function};

    #[test]
    fn t1() {
        let path = Path::new("../data/hello_kern.o");
        let buffer = fs::read(path).unwrap();
        let elf = match Object::parse(&buffer).unwrap() {
            Object::Elf(elf) => elf,
            _ => panic!(""),
        };

        let r = locate_function(&elf, "bpf_prog").unwrap();

        let ops = &buffer[r];

        let instructions = Instructions::from(ops);
        println!("{:?}", instructions);
        // disassemble(ops);
    }
}
