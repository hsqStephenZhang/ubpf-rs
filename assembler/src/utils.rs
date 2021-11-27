// register
pub fn reg(reg: u8) -> String {
    return format!("r{}", reg);
}

// immediate
pub fn imm(reg: i32) -> String {
    return format!("{}", reg);
}

// memory
pub fn memory(base: &str, off: i16) -> String {
    if off != 0 {
        return format!("{}{}", base, offset(off));
    } else {
        return format!("{}", base);
    }
}

// operation
pub fn offset(off: i16) -> String {
    return format!("{}", off);
}
