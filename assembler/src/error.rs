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
