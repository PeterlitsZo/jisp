pub struct Bytecode {
    bytes: Vec<u8>,
}

impl Bytecode {
    /// Create a new [Bytecode].
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    /// Push a byte.
    pub fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Push some bytes.
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend(bytes);
    }

    /// Return the view of bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Op {
    PushInt,
    PushBool,
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Ret,
}

impl Op {
    const PUSH_INT: u8 = 0x00;
    const PUSH_BOOL: u8 = 0x01;
    const POP: u8 = 0x02;
    const ADD: u8 = 0x08;
    const SUB: u8 = 0x09;
    const MUL: u8 = 0x0a;
    const DIV: u8 = 0x0b;
    const MOD: u8 = 0x0c;
    const EQ: u8 = 0x10;
    const NE: u8 = 0x11;
    const LT: u8 = 0x12;
    const LE: u8 = 0x13;
    const GT: u8 = 0x14;
    const GE: u8 = 0x15;
    const RET: u8 = 0x18;

    pub fn display(&self) -> &'static str {
        match self {
            Self::PushInt => "PUSH_INT",
            Self::PushBool => "PUSH_BOOL",
            Self::Pop => "POP",
            Self::Add => "ADD",
            Self::Sub => "SUB",
            Self::Mul => "MUL",
            Self::Div => "DIV",
            Self::Mod => "MOD",
            Self::Eq => "EQ",
            Self::Ne => "NE",
            Self::Lt => "LT",
            Self::Le => "LE",
            Self::Gt => "GT",
            Self::Ge => "GE",
            Self::Ret => "RET",
        }
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            Self::PUSH_INT => Some(Self::PushInt),
            Self::PUSH_BOOL => Some(Self::PushBool),
            Self::POP => Some(Self::Pop),
            Self::ADD => Some(Self::Add),
            Self::SUB => Some(Self::Sub),
            Self::MUL => Some(Self::Mul),
            Self::DIV => Some(Self::Div),
            Self::MOD => Some(Self::Mod),
            Self::EQ => Some(Self::Eq),
            Self::NE => Some(Self::Ne),
            Self::LT => Some(Self::Lt),
            Self::LE => Some(Self::Le),
            Self::GT => Some(Self::Gt),
            Self::GE => Some(Self::Ge),
            Self::RET => Some(Self::Ret),
            _ => None,
        }
    }

    pub fn byte(self) -> u8 {
        match self {
            Self::PushInt => Self::PUSH_INT,
            Self::PushBool => Self::PUSH_BOOL,
            Self::Pop => Self::POP,
            Self::Add => Self::ADD,
            Self::Sub => Self::SUB,
            Self::Mul => Self::MUL,
            Self::Div => Self::DIV,
            Self::Mod => Self::MOD,
            Self::Eq => Self::EQ,
            Self::Ne => Self::NE,
            Self::Lt => Self::LT,
            Self::Le => Self::LE,
            Self::Gt => Self::GT,
            Self::Ge => Self::GE,
            Self::Ret => Self::RET,
        }
    }

    pub fn op_len(self) -> usize {
        match self {
            Self::PushInt => 9,
            Self::PushBool => 2,
            Self::Pop => 1,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod => 1,
            Self::Eq | Self::Ne | Self::Lt | Self::Le | Self::Gt | Self::Ge => 1,
            Self::Ret => 1,
        }
    }
}