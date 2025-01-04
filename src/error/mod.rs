#![allow(dead_code)] // TODO (PeterlitsZo): This module will be used in the future.

use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    /// The bytecode is empty - and cannot be run by runner.
    #[error("empty bytecode")]
    EmptyBytecode,

    /// Unknown opcode.
    #[error("unknown opcode: {0}")]
    UnknownOpcode(u8),
}

type Result<T> = std::result::Result<T, Error>;