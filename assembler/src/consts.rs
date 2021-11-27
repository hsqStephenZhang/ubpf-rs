pub mod class {
    pub const LD: u8 = 0;
    pub const LDX: u8 = 1;
    pub const ST: u8 = 2;
    pub const STX: u8 = 3;
    pub const ALU: u8 = 4;
    pub const JMP: u8 = 5;
    pub const RET: u8 = 6;
    pub const ALU64: u8 = 7;
}

pub mod alu {

    pub const NEG: u8 = 8;
    pub const END: u8 = 13;
}
