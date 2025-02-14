use std::fmt::Debug;

use thiserror::Error;

use crate::asm::Label;
use crate::bytecode::Op;
use crate::value::ValueKind;

#[derive(Debug, Error, PartialEq, Eq)]
pub(super) enum Error {
    /// Unknown label - bad ASM.
    #[error("unknown label: {0:?}")]
    UnknownLabel(Label),

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

    /// Type error - the given op cannot handle the arguments.
    #[allow(clippy::enum_variant_names)]
    #[error("type error: unsupported operand type(s) for {op:?}: {arg_kinds:?}")]
    TypeError { op: Op, arg_kinds: ArgKinds },
}

/// The kinds of the arguments.  Length will always greater than 0 (never
/// empty).
/// 
/// Used to show error message better.
#[derive(PartialEq, Eq)]
pub(super) struct ArgKinds(Vec<ValueKind>);

impl ArgKinds {
    /// Create a new [ArgKinds] by arguments.
    /// 
    /// # Panic
    /// 
    /// If argument kinds are empty, it will panic.
    pub(super) fn new(kinds: Vec<ValueKind>) -> Self {
        if kinds.is_empty() {
            panic!("kinds are empty")
        }
        Self(kinds)
    }
}

impl Debug for ArgKinds {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        result.push_str(&format!("{:?}", self.0[0]));
        for i in 1..self.0.len() {
            if i == self.0.len() - 1 {
                result.push_str(" and ");
            } else {
                result.push_str(", ");
            }
            result.push_str(&format!("{:?}", self.0[i]));
        }
        write!(f, "{}", result)
    }
}

pub(super) type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_format() {
        let err = Error::TypeError {
            op: Op::Add,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Int])
        };
        let err = err.to_string();
        assert_eq!(err, "type error: unsupported operand type(s) for Add: Null and Int")
    }
}