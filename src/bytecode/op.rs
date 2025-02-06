//! The bytecode opcodes.  See [Op] to know more.

/// The bytecode opcodes.
/// 
/// You can see the [crate::asm::Stat] to know more about those opcodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Op {
    Return,

    LoadNull,
    LoadInt,
    LoadFloat,
    LoadBool,

    Pop,

    Add,
    Sub,
    Mul,
    Div,
    Rem,
    FloorDiv,

    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

impl Op {
    const OP_RETURN: u8 = Self::Return as u8;

    const OP_LOAD_NULL: u8 = Self::LoadNull as u8;
    const OP_LOAD_INT: u8 = Self::LoadInt as u8;
    const OP_LOAD_FLOAT: u8 = Self::LoadFloat as u8;
    const OP_LOAD_BOOL: u8 = Self::LoadBool as u8;

    const OP_POP: u8 = Self::Pop as u8;

    const OP_ADD: u8 = Self::Add as u8;
    const OP_SUB: u8 = Self::Sub as u8;
    const OP_MUL: u8 = Self::Mul as u8;
    const OP_DIV: u8 = Self::Div as u8;
    const OP_REM: u8 = Self::Rem as u8;
    const OP_FLOOR_DIV: u8 = Self::FloorDiv as u8;

    const OP_EQ: u8 = Self::Eq as u8;
    const OP_NE: u8 = Self::Ne as u8;
    const OP_LT: u8 = Self::Lt as u8;
    const OP_LE: u8 = Self::Le as u8;
    const OP_GT: u8 = Self::Gt as u8;
    const OP_GE: u8 = Self::Ge as u8;

    /// Convert the opcode to a byte.
    pub(crate) fn into_u8(self) -> u8 {
        match self {
            Self::Return => Self::OP_RETURN,

            Self::LoadNull => Self::OP_LOAD_NULL,
            Self::LoadInt => Self::OP_LOAD_INT,
            Self::LoadFloat => Self::OP_LOAD_FLOAT,
            Self::LoadBool => Self::OP_LOAD_BOOL,

            Self::Pop => Self::OP_POP,

            Self::Add => Self::OP_ADD,
            Self::Sub => Self::OP_SUB,
            Self::Mul => Self::OP_MUL,
            Self::Div => Self::OP_DIV,
            Self::Rem => Self::OP_REM,
            Self::FloorDiv => Self::OP_FLOOR_DIV,

            Self::Eq => Self::OP_EQ,
            Self::Ne => Self::OP_NE,
            Self::Lt => Self::OP_LT,
            Self::Le => Self::OP_LE,
            Self::Gt => Self::OP_GT,
            Self::Ge => Self::OP_GE,
        }
    }

    /// Convert a byte to an opcode.
    pub(crate) fn from_u8(byte: u8) -> Option<Self> {
        match byte {
            Self::OP_RETURN => Some(Self::Return),

            Self::OP_LOAD_NULL => Some(Self::LoadNull),
            Self::OP_LOAD_INT => Some(Self::LoadInt),
            Self::OP_LOAD_FLOAT => Some(Self::LoadFloat),
            Self::OP_LOAD_BOOL => Some(Self::LoadBool),

            Self::OP_POP => Some(Self::Pop),

            Self::OP_ADD => Some(Self::Add),
            Self::OP_SUB => Some(Self::Sub),
            Self::OP_MUL => Some(Self::Mul),
            Self::OP_DIV => Some(Self::Div),
            Self::OP_REM => Some(Self::Rem),
            Self::OP_FLOOR_DIV => Some(Self::FloorDiv),

            Self::OP_EQ => Some(Self::Eq),
            Self::OP_NE => Some(Self::Ne),
            Self::OP_LT => Some(Self::Lt),
            Self::OP_LE => Some(Self::Le),
            Self::OP_GT => Some(Self::Gt),
            Self::OP_GE => Some(Self::Ge),

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
            Self::LoadBool => 2,

            Self::Pop => 1,

            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 1,
            Self::Div => 1,
            Self::Rem => 1,
            Self::FloorDiv => 1,

            Self::Eq => 1,
            Self::Ne => 1,
            Self::Lt => 1,
            Self::Le => 1,
            Self::Gt => 1,
            Self::Ge => 1,
        }
    }
}