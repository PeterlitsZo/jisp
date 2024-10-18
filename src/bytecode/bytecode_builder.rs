use crate::asm::{Asm, AsmStatement};

use super::{ins, Bytecode};

pub struct BytecodeBuilder {
    asm: Asm,
}

impl BytecodeBuilder {
    pub fn new(asm: Asm) -> Self {
        Self { asm }
    }

    pub fn build(self) -> Bytecode {
        let mut bytecode = Bytecode::new();
        for stmt in self.asm.statements() {
            match stmt {
                AsmStatement::PushI64 { val } => {
                    bytecode.push_byte(ins::PUSH_I64);
                    bytecode.push_bytes(&val.to_le_bytes());
                },
                AsmStatement::Ret => bytecode.push_byte(ins::RET),
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
            ins::PUSH_I64, 0xff, 0, 0, 0, 0, 0, 0, 0,
            ins::RET,
        ]));
    }
}