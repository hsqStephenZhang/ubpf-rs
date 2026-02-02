// register format
pub fn reg(reg: u8) -> String {
    format!("r{}", reg)
}

// memory format
pub fn memory(base: &str, off: i16) -> String {
    if off != 0 {
        format!("{}{:+}", base, off)
    } else {
        base.to_string()
    }
}
