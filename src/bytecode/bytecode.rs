use crate::value::Value;

/// The bytecode.
#[derive(Debug, PartialEq, Eq)]
pub struct Bytecode {
    pub consts: Vec<Value>, // The all consts.
    pub fns: Vec<BytecodeFn>, // The functions.
}

impl Bytecode {
    /// Build a empty [Bytecode].
    pub fn new() -> Self {
        Self {
            consts: vec![],
            fns: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BytecodeFn {
    pub locals: u32, // The number of local variables.
    bytes: Vec<u8>,
}

impl BytecodeFn {
    /// Build a empty [BytecodeFn].
    pub fn new() -> Self {
        Self { locals: 0, bytes: vec![] }
    }

    /// Build a [BytecodeFn].
    pub fn from<T: Into<Vec<u8>>>(locals: u32, bytes: T) -> Self {
        Self { locals, bytes: bytes.into() }
    }

    /// Push one byte to [Bytecode].
    pub fn push_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    /// Push some bytes to [Bytecode].
    pub fn push_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }

    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
}