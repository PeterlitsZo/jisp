use thiserror::Error;

#[derive(Debug, Error)]
pub(super) enum Error {
    /// The bytecode is empty - and cannot be run by runner.
    #[error("empty bytecode")]
    EmptyBytecode,

    /// Unknown opcode.
    #[error("unknown opcode: {0}")]
    UnknownOpcode(u8),
}

pub(super) type Result<T> = std::result::Result<T, Error>;