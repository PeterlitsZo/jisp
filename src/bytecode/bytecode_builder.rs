use std::collections::HashMap;

use crate::{asm::{Asm, AsmStatement}, bytecode::bytecode::BytecodeFn};

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

        let mut bytecode = Bytecode::new();

        for func in &self.asm.ifns {
            let mut label_to_offset = HashMap::new();
            let mut cur_offset = 0u32;
            for stmt in &func.statements {
                match stmt {
                    AS::Label { label } => {
                        label_to_offset.insert(label.clone(), cur_offset);
                    }

                    AS::Ret | AS::Add | AS::Sub | AS::Mul | AS::Div | AS::Eq |
                    AS::Ne | AS::Lt | AS::Le | AS::Gt | AS::Ge | AS::Pop => {
                        cur_offset += 1;
                    }

                    AS::PushI64 { val: _ } => {
                        cur_offset += 1 + 8;
                    }

                    AS::Load { index: _ } | AS::Store { index: _ } |
                    AS::Jump { label: _ } | AS::JumpFalse { label: _ } |
                    AS::PushConst { index: _ } | AS::Call { args: _ } => {
                        cur_offset += 1 + 4;
                    }
                }
            }

            let mut bcfn = BytecodeFn::new();
            bcfn.locals = func.locals;
            for stmt in &func.statements {
                match stmt {
                    AS::Label { label: _ } => (),

                    AS::Ret => bcfn.push_byte(ins::RET),

                    AS::PushI64 { val } => {
                        bcfn.push_byte(ins::PUSH_I64);
                        bcfn.push_bytes(&val.to_le_bytes());
                    },
                    AS::PushConst { index } => {
                        bcfn.push_byte(ins::PUSH_CONST);
                        bcfn.push_bytes(&index.to_le_bytes());
                    }
                    AS::Pop => bcfn.push_byte(ins::POP),

                    AS::Add => bcfn.push_byte(ins::ADD),
                    AS::Sub => bcfn.push_byte(ins::SUB),
                    AS::Mul => bcfn.push_byte(ins::MUL),
                    AS::Div => bcfn.push_byte(ins::DIV),
                    AS::Eq => bcfn.push_byte(ins::EQ),
                    AS::Ne => bcfn.push_byte(ins::NE),
                    AS::Lt => bcfn.push_byte(ins::LT),
                    AS::Le => bcfn.push_byte(ins::LE),
                    AS::Gt => bcfn.push_byte(ins::GT),
                    AS::Ge => bcfn.push_byte(ins::GE),

                    AS::Load { index } => {
                        bcfn.push_byte(ins::LOAD);
                        bcfn.push_bytes(&index.to_le_bytes());
                    },
                    AS::Store { index } => {
                        bcfn.push_byte(ins::STORE);
                        bcfn.push_bytes(&index.to_le_bytes());
                    },

                    AS::Jump { label } => {
                        let offset = label_to_offset[&label];
                        bcfn.push_byte(ins::JUMP);
                        bcfn.push_bytes(&offset.to_le_bytes());
                    },
                    AS::JumpFalse { label } => {
                        let offset = label_to_offset[&label];
                        bcfn.push_byte(ins::JUMP_FALSE);
                        bcfn.push_bytes(&offset.to_le_bytes());
                    },

                    AS::Call { args: num } => {
                        bcfn.push_byte(ins::CALL);
                        bcfn.push_bytes(&num.to_le_bytes());
                    }
                }
            }
            bytecode.ifns.push(bcfn);
        }
        
        bytecode.consts = self.asm.consts;
        bytecode.xfns = self.asm.xfns;
        bytecode
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::{AsmFn, AsmLabel}, value::Value};

    use super::*;

    #[test]
    fn basic() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ]));
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        let mut wanted = Bytecode::new();
        wanted.ifns.push(BytecodeFn::from(0, [
            ins::PUSH_I64, 0xff, 0, 0, 0, 0, 0, 0, 0,
            ins::RET,
        ]));
        assert_eq!(bytecode, wanted);

        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        let mut wanted = Bytecode::new();
        wanted.ifns.push(BytecodeFn::from(0, [
            ins::PUSH_I64, 0x01, 0, 0, 0, 0, 0, 0, 0,
            ins::PUSH_I64, 0x02, 0, 0, 0, 0, 0, 0, 0,
            ins::ADD,
            ins::RET,
        ]));
        assert_eq!(bytecode, wanted);
    }

    #[test]
    fn label_jump() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
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
        ]));
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        let mut wanted = Bytecode::new();
        wanted.ifns.push(BytecodeFn::from(0, [
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
        assert_eq!(bytecode, wanted);
    }

    #[test]
    fn string() {
        let mut asm = Asm::new();
        asm.consts = vec![
            Value::Str("hello".to_string()),
        ];
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Ret,
        ]));
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        let mut wanted = Bytecode::new();
        wanted.consts = vec![
            Value::Str("hello".to_string())
        ];
        wanted.ifns.push(BytecodeFn::from(0, [
            /* off: 0x00 = 00 */ ins::PUSH_CONST, 0x00, 0, 0, 0,
            /* off: 0x05 = 05 */ ins::RET,
        ]));
        assert_eq!(bytecode, wanted);
    }
}