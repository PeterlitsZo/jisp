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
                AsmStatement::Ret => bytecode.push_byte(ins::RET),

                AsmStatement::PushI64 { val } => {
                    bytecode.push_byte(ins::PUSH_I64);
                    bytecode.push_bytes(&val.to_le_bytes());
                },

                AsmStatement::Add => bytecode.push_byte(ins::ADD),
                AsmStatement::Sub => bytecode.push_byte(ins::SUB),
                AsmStatement::Mul => bytecode.push_byte(ins::MUL),
                AsmStatement::Div => bytecode.push_byte(ins::DIV),
                AsmStatement::Eq => bytecode.push_byte(ins::EQ),
                AsmStatement::Ne => bytecode.push_byte(ins::NE),
                AsmStatement::Lt => bytecode.push_byte(ins::LT),
                AsmStatement::Le => bytecode.push_byte(ins::LE),
                AsmStatement::Gt => bytecode.push_byte(ins::GT),
                AsmStatement::Ge => bytecode.push_byte(ins::GE),
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

        let asm = Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]);
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        assert_eq!(bytecode, Bytecode::from([
            ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            ins::PUSH_I64, 0x02, 0, 0, 0, 0, 0, 0, 0,
            ins::ADD,
            ins::RET,
        ]));
    }
}