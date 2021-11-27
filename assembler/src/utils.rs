// register
pub fn reg(reg: u8) -> String {
    return format!("r{}", reg);
}

// immediate
pub fn imm(reg: u32) -> String {
    return format!("#{:X}", reg);
}

// memory
pub fn memory(base: &str, off: u16) -> String {
    if off != 0 {
        return format!("{}{}", base, offset(off));
    } else {
        return format!("{}", base);
    }
}

// operation
pub fn offset(off: u16) -> String {
    if off <= 32767 {
        return format!("+{}", off);
    } else {
        let r= 65536u32 - off as u32;
        return format!("-{}", r);
    }
}
