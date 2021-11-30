// register
pub fn reg(reg: u8) -> String {
    return format!("r{}", reg);
}

// memory
pub fn memory(base: &str, off: i16) -> String {
    if off != 0 {
        return format!("{}{:+}", base, off);
    } else {
        return format!("{}", base);
    }
}
