use std::collections::HashMap;

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
        type AS = AsmStatement;

        let mut bytecode = Bytecode::new(self.asm.locals);

        let mut label_to_offset = HashMap::new();
        let mut cur_offset = 0u32;
        for stmt in self.asm.statements() {
            match stmt {
                AS::Label { label } => {
                    label_to_offset.insert(label.clone(), cur_offset);
                }

                AS::Ret | AS::Add | AS::Sub | AS::Mul | AS::Div | AS::Eq |
                AS::Ne | AS::Lt | AS::Le | AS::Gt | AS::Ge => {
                    cur_offset += 1;
                }

                AS::PushI64 { val: _ } => {
                    cur_offset += 1 + 8;
                }

                AS::Load { index: _ } | AS::Store { index: _ } |
                AS::Jump { label: _ } | AS::JumpFalse { label: _ } => {
                    cur_offset += 1 + 4;
                }
            }
        }
        for stmt in self.asm.statements() {
            match stmt {
                AS::Label { label: _ } => (),

                AS::Ret => bytecode.push_byte(ins::RET),

                AS::PushI64 { val } => {
                    bytecode.push_byte(ins::PUSH_I64);
                    bytecode.push_bytes(&val.to_le_bytes());
                },

                AS::Add => bytecode.push_byte(ins::ADD),
                AS::Sub => bytecode.push_byte(ins::SUB),
                AS::Mul => bytecode.push_byte(ins::MUL),
                AS::Div => bytecode.push_byte(ins::DIV),
                AS::Eq => bytecode.push_byte(ins::EQ),
                AS::Ne => bytecode.push_byte(ins::NE),
                AS::Lt => bytecode.push_byte(ins::LT),
                AS::Le => bytecode.push_byte(ins::LE),
                AS::Gt => bytecode.push_byte(ins::GT),
                AS::Ge => bytecode.push_byte(ins::GE),

                AS::Load { index } => {
                    bytecode.push_byte(ins::LOAD);
                    bytecode.push_bytes(&index.to_le_bytes());
                },
                AS::Store { index } => {
                    bytecode.push_byte(ins::STORE);
                    bytecode.push_bytes(&index.to_le_bytes());
                },

                AS::Jump { label } => {
                    let offset = label_to_offset[label];
                    bytecode.push_byte(ins::JUMP);
                    bytecode.push_bytes(&offset.to_le_bytes());
                },
                AS::JumpFalse { label } => {
                    let offset = label_to_offset[label];
                    bytecode.push_byte(ins::JUMP_FALSE);
                    bytecode.push_bytes(&offset.to_le_bytes());
                },
            }
        }
        bytecode
    }
}

#[cfg(test)]
mod tests {
    use crate::asm::AsmLabel;

    use super::*;

    #[test]
    fn basic() {
        let asm = Asm::from(0, [
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ]);
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        assert_eq!(bytecode, Bytecode::from(0, [
            ins::PUSH_I64, 0xff, 0, 0, 0, 0, 0, 0, 0,
            ins::RET,
        ]));

        let asm = Asm::from(0, [
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]);
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        assert_eq!(bytecode, Bytecode::from(0, [
            ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            ins::PUSH_I64, 0x02, 0, 0, 0, 0, 0, 0, 0,
            ins::ADD,
            ins::RET,
        ]));
    }

    #[test]
    fn label_jump() {
        let asm = Asm::from(0, [
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Mul,
            AsmStatement::Label { label: AsmLabel::new(".L2") },
            AsmStatement::Ret,
        ]);
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        assert_eq!(bytecode, Bytecode::from(0, [
            /* off: 0x00 = 00 */ ins::PUSH_I64, 0x02, 0, 0, 0, 0, 0, 0, 0,
            /* off: 0x09 = 09 */ ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            /* off: 0x12 = 18 */ ins::EQ,
            /* off: 0x13 = 19 */ ins::JUMP_FALSE, 0x26, 0, 0, 0,
            /* off: 0x18 = 24 */ ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            /* off: 0x21 = 33 */ ins::JUMP, 0x39, 0, 0, 0,
            /* off: 0x26 = 38 */ ins::PUSH_I64, 0x02, 0, 0, 0, 0, 0, 0, 0,
            /* off: 0x2f = 47 */ ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            /* off: 0x38 = 56 */ ins::MUL,
            /* off: 0x39 = 57 */ ins::RET,
        ]));
    }
}