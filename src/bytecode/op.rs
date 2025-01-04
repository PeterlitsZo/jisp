//! The bytecode opcodes.  See [Op] to know more.

/// The bytecode opcodes.
#[derive(Debug, Clone, Copy)]
pub(super) enum Op {
    Return,

    LoadNull,
}

impl Op {
    const OP_RETURN: u8 = Self::Return as u8;

    const OP_LOAD_NULL: u8 = Self::LoadNull as u8;

    /// Convert the opcode to a byte.
    pub(super) fn into_u8(self) -> u8 {
        match self {
            Self::Return => Self::OP_RETURN,
            Self::LoadNull => Self::OP_LOAD_NULL,
        }
    }

    /// Convert a byte to an opcode.
    fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            Self::OP_RETURN => Some(Self::Return),
            Self::OP_LOAD_NULL => Some(Self::LoadNull),
            _ => None,
        }
    }

    /// Get the opcode's length.
    fn op_len(&self) -> usize {
        match self {
            Self::Return => 1,
            Self::LoadNull => 1,
        }
    }
}