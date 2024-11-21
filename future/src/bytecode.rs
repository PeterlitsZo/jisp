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
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Ret,
}

impl Op {
    const PUSH_INT: u8 = 0x00;
    const POP: u8 = 0x01;
    const ADD: u8 = 0x08;
    const SUB: u8 = 0x09;
    const MUL: u8 = 0x0a;
    const DIV: u8 = 0x0b;
    const MOD: u8 = 0x0c;
    const RET: u8 = 0x10;

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            Self::PUSH_INT => Some(Self::PushInt),
            Self::POP => Some(Self::Pop),
            Self::ADD => Some(Self::Add),
            Self::SUB => Some(Self::Sub),
            Self::MUL => Some(Self::Mul),
            Self::DIV => Some(Self::Div),
            Self::MOD => Some(Self::Mod),
            Self::RET => Some(Self::Ret),
            _ => None,
        }
    }

    pub fn byte(self) -> u8 {
        match self {
            Self::PushInt => Self::PUSH_INT,
            Self::Pop => Self::POP,
            Self::Add => Self::ADD,
            Self::Sub => Self::SUB,
            Self::Mul => Self::MUL,
            Self::Div => Self::DIV,
            Self::Mod => Self::MOD,
            Self::Ret => Self::RET,
        }
    }

    pub fn op_len(self) -> usize {
        match self {
            Self::PushInt => 9,
            Self::Pop => 1,
            Self::Add | Self::Sub | Self::Mul | Self::Div | Self::Mod => 1,
            Self::Ret => 1,
        }
    }
}