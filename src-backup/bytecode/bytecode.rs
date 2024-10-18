use super::asm::{Asm, AsmStatement};

#[derive(Debug, PartialEq, Eq)]
pub struct Bytecode {
    pub bytes: Vec<u8>,
}

impl Bytecode {
    pub fn new() -> Self {
        Self { bytes: vec![] }
    }

    pub fn from<T>(bytes: T) -> Self where T: Into<Vec<u8>> {
        Self { bytes: bytes.into() }
    }

    fn append_byte(&mut self, byte: u8) {
        self.bytes.push(byte);
    }

    fn append_bytes(&mut self, bytes: &[u8]) {
        self.bytes.extend_from_slice(bytes);
    }
}

pub const RET: u8 = 0x00;
pub const PUSH_I64: u8 = 0x01;
pub const ADD_I64: u8 = 0x02;
pub const SUB_I64: u8 = 0x03;

pub struct BytecodeBuilder {
    asm: Asm,
}

impl BytecodeBuilder {
    pub fn new(asm: Asm) -> Self {
        Self { asm }
    }

    pub fn build(&self) -> Bytecode {
        let mut bytecode = Bytecode::new();
        for stmt in &self.asm.statements {
            match stmt {
                AsmStatement::PushI64 { val } => {
                    bytecode.append_byte(PUSH_I64);
                    bytecode.append_bytes(&val.to_le_bytes());
                },
                AsmStatement::Ret => bytecode.append_byte(RET),
                AsmStatement::AddI64 => bytecode.append_byte(ADD_I64),
                AsmStatement::SubI64 => bytecode.append_byte(SUB_I64),
            }
        }
        bytecode
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let asm = Asm::from([
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ]);
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        assert_eq!(bytecode, Bytecode::from([
            PUSH_I64, 0xff, 0, 0, 0, 0, 0, 0, 0,
            RET,
        ]));
    }
}