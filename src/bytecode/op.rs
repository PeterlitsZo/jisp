//! The bytecode opcodes.  See [Op] to know more.

/// The bytecode opcodes.
#[derive(Debug, Clone, Copy)]
pub(crate) enum Op {
    Return,

    LoadNull,
    LoadInt,
    LoadFloat,

    Pop,
}

impl Op {
    const OP_RETURN: u8 = Self::Return as u8;

    const OP_LOAD_NULL: u8 = Self::LoadNull as u8;
    const OP_LOAD_INT: u8 = Self::LoadInt as u8;
    const OP_LOAD_FLOAT: u8 = Self::LoadFloat as u8;

    const OP_POP: u8 = Self::Pop as u8;

    /// Convert the opcode to a byte.
    pub(crate) fn into_u8(self) -> u8 {
        match self {
            Self::Return => Self::OP_RETURN,

            Self::LoadNull => Self::OP_LOAD_NULL,
            Self::LoadInt => Self::OP_LOAD_INT,
            Self::LoadFloat => Self::OP_LOAD_FLOAT,

            Self::Pop => Self::OP_POP,
        }
    }

    /// Convert a byte to an opcode.
    pub(crate) fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            Self::OP_RETURN => Some(Self::Return),

            Self::OP_LOAD_NULL => Some(Self::LoadNull),
            Self::OP_LOAD_INT => Some(Self::LoadInt),
            Self::OP_LOAD_FLOAT => Some(Self::LoadFloat),

            Self::OP_POP => Some(Self::Pop),

            _ => None,
        }
    }

    /// Get the opcode's length.
    pub(crate) fn op_len(&self) -> usize {
        match self {
            Self::Return => 1,

            Self::LoadNull => 1,
            Self::LoadInt => 9,
            Self::LoadFloat => 9,

            Self::Pop => 1,
        }
    }
}