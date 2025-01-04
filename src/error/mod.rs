use thiserror::Error;

use crate::bytecode::Op;

#[derive(Debug, Error)]
pub(super) enum Error {
    /// The bytecode is empty - and cannot be run by runner.
    #[error("empty bytecode")]
    EmptyBytecode,

    /// Unknown opcode.
    #[error("unknown opcode: {0}")]
    UnknownOpcode(u8),

    /// Bad opcode.
    #[error("bad opcode: {0:?}")]
    BadOpcode(Op),

    /// There is no frame to run - bytecode is bad or internal error.
    #[error("no frame to run")]
    NoFrameToRun,

    /// The stack is empty - and cannot pop.
    #[error("empty stack")]
    EmptyStack,
}

pub(super) type Result<T> = std::result::Result<T, Error>;