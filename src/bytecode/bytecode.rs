/// The bytecode.
#[derive(Debug, PartialEq, Eq)]
pub struct Bytecode {
    pub locals: u32, // The number of local variables.
    bytes: Vec<u8>,
}

impl Bytecode {
    /// Build a empty [Bytecode].
    pub fn new(locals: u32) -> Self {
        Self {
            locals,
            bytes: vec![]
        }
    }

    /// Build a non-empty [Bytecode] from some bytes.
    #[cfg(test)]
    pub fn from<T>(locals: u32, bytes: T) -> Self where T: Into<Vec<u8>> {
        Self {
            locals,
            bytes: bytes.into()
        }
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