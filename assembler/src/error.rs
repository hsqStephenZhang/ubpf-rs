use thiserror::Error;

#[derive(Error, Debug)]
pub enum ElfError {
    #[error("platform not support")]
    PlatFormNotSupport,
    #[error("make exec error")]
    MakeExec,
    #[error("no text")]
    NoTextSection,
    #[error("section not found")]
    SectionNotFound(String),
    #[error("section not found")]
    FunctionNotFound(String),
    #[error("unknown data store error")]
    Unknown,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("nom parse error")]
    ParseFailed,
    #[error("invalid dst register")]
    InvalidDst(i64),
    #[error("invalid src register")]
    InvalidSrc(i64),
    #[error("invalid offset")]
    InvalidOffset(i64),
    #[error("invalid immediate")]
    InvalidImmediate(i64),
}
