use thiserror::Error;

#[derive(Error, Debug)]
pub enum VmError {
    #[error("div zero")]
    DivZero,
    #[error("virtual memory set failed, out of boundary")]
    MemOutOfBound,
    #[error("unknown virtual machine error")]
    Unknown,
}