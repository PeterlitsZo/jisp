//! The builders for [Bytecode] and [IFunc].

use std::{collections::BTreeMap, mem::swap};

use crate::{asm::{Asm, Label, Stat}, error::Error};

use super::{op::Op, Bytecode, IFunc};

/// The [Bytecode] builder.
pub(crate) struct BytecodeBuilder {
    bytecode: Bytecode,
}

impl BytecodeBuilder {
    /// Create a new [BytecodeBuilder].
    pub(super) fn new() -> Self {
        Self {
            bytecode: Bytecode::new(),
        }
    }

    /// Parse the [Asm].
    pub(crate) fn parse_asm(&mut self, asm: &Asm) -> Result<&mut Self, Error> {
        for asm_ifunc in asm.ifuncs() {
            let mut ifunc_builder = IFunc::builder();
            for stat in asm_ifunc.stats() {
                ifunc_builder.push_stat(stat);
            }
            self.bytecode.ifuncs.push(ifunc_builder.build()?);
        }

        Ok(self)
    }

    /// Build the [Bytecode] and reset the builder itself.
    pub(crate) fn build(&mut self) -> Bytecode {
        let mut tmp_bytecode = Bytecode::new();
        swap(&mut tmp_bytecode, &mut self.bytecode);
        tmp_bytecode
    }
}

/// The core of the [IFuncBuilder].
struct IFuncBuilderCore {
    ifunc: IFunc,

    cur_addr: usize,
    label_to_addr: BTreeMap<Label, usize>,
}

type Hook = Box<dyn Fn(&mut IFuncBuilderCore) -> Result<(), Error>>;

/// The [IFunc] builder.
pub(super) struct IFuncBuilder {
    core: IFuncBuilderCore,

    before_build_hooks: Vec<Hook>,
}

impl IFuncBuilder {
    /// Create a new [IFuncBuilder].
    pub(super) fn new() -> Self {
        Self {
            core: IFuncBuilderCore {
                ifunc: IFunc::new(),

                cur_addr: 0,
                label_to_addr: BTreeMap::new(),
            },
            before_build_hooks: vec![],
        }
    }

    /// Push a [Stat].
    fn push_stat(&mut self, stat: &Stat) -> &mut Self {
        match stat {
            Stat::Label(label) => self.mem_label(label),

            Stat::Return => self.push_op(Op::Return),

            Stat::StoreLocal(_) => todo!(),
            Stat::LoadLocal(_) => todo!(),

            Stat::LoadNull => self.push_op(Op::LoadNull),
            Stat::LoadInt(val) => self.push_op_args(Op::LoadInt, &val.to_le_bytes()),
            Stat::LoadFloat(val) => self.push_op_args(Op::LoadFloat, &val.to_le_bytes()),
            Stat::LoadBool(val) => self.push_op_args(Op::LoadBool, &[if *val { 1 } else { 0 }]),

            Stat::Pop => self.push_op(Op::Pop),

            Stat::JumpIfTrue(label) => self.push_op_label(Op::JumpIfTrue, label),
            Stat::JumpIfFalse(label) => self.push_op_label(Op::JumpIfFalse, label),
            Stat::Jump(label) => self.push_op_label(Op::Jump, label),

            Stat::Add => self.push_op(Op::Add),
            Stat::Sub => self.push_op(Op::Sub),
            Stat::Mul => self.push_op(Op::Mul),
            Stat::Div => self.push_op(Op::Div),
            Stat::Rem => self.push_op(Op::Rem),
            Stat::FloorDiv => self.push_op(Op::FloorDiv),

            Stat::Eq => self.push_op(Op::Eq),
            Stat::Ne => self.push_op(Op::Ne),
            Stat::Lt => self.push_op(Op::Lt),
            Stat::Le => self.push_op(Op::Le),
            Stat::Gt => self.push_op(Op::Gt),
            Stat::Ge => self.push_op(Op::Ge),

            Stat::Not => self.push_op(Op::Not),
            Stat::And => self.push_op(Op::And),
            Stat::Or => self.push_op(Op::Or),
        }
    }

    /// Push an opcode without arguments.
    fn push_op(&mut self, op: Op) -> &mut Self {
        self.core.ifunc.code.push(op.into_u8());
        self.core.cur_addr += op.op_len();
        self
    }

    /// Push an opcode with arguments.
    fn push_op_args(&mut self, op: Op, args: &[u8]) -> &mut Self {
        self.core.ifunc.code.push(op.into_u8());
        self.core.ifunc.code.extend(args);
        self.core.cur_addr += op.op_len();
        self
    }

    /// Push an opcode with label.
    fn push_op_label(&mut self, op: Op, label: &Label) -> &mut Self {
        self.core.ifunc.code.push(op.into_u8());
        if let Some(addr) = self.core.label_to_addr.get(label) {
            self.core.ifunc.code.extend((*addr as u64).to_le_bytes());
        } else {
            self.core.ifunc.code.extend(&0u64.to_le_bytes());

            let label_hole_addr = self.core.cur_addr + 1;
            let label_moved = label.clone();
            self.before_build_hooks.push(Box::new(move |ifunc_builder_core| {
                if let Some(&addr) = ifunc_builder_core.label_to_addr.get(&label_moved) {
                    ifunc_builder_core.ifunc.code[label_hole_addr..label_hole_addr+8]
                        .clone_from_slice(&(addr as u64).to_le_bytes());
                } else {
                    return Err(Error::UnknownLabel(label_moved.clone()));
                }
                Ok(())
            }));
        }
        self.core.cur_addr += op.op_len();
        self
    }

    /// Memorize the label and its address.
    fn mem_label(&mut self, label: &Label) -> &mut Self {
        self.core.label_to_addr.insert(label.clone(), self.core.cur_addr);
        self
    }

    /// Build the [IFunc] and reset the builder itself.
    /// 
    /// If error raised, the state of the builder is unclear (maybe or maybe not reset).
    fn build(&mut self) -> Result<IFunc, Error> {
        // Call the before build hooks (for example, set the label argument with
        // new memory).
        for hook in &self.before_build_hooks {
            hook(&mut self.core)?;
        }

        // Reset self and return the built IFunc.
        let mut tmp_ifunc = IFunc::new();
        swap(&mut tmp_ifunc, &mut self.core.ifunc);
        self.core.cur_addr = 0;
        self.core.label_to_addr.clear();
        Ok(tmp_ifunc)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::asm::Asm;
    use crate::bytecode::op::Op;

    use super::*;

    fn build_bytecode_from_asm(asm: &str) -> Result<Bytecode, Error> {
        let asm = Asm::from_program(asm);
        Ok(Bytecode::builder().parse_asm(&asm)?.build())
    }

    #[test]
    fn test_build_from_simple_asm() {
        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                LOAD_NULL
                POP
                LOAD_INT        42
                POP
                LOAD_FLOAT      3.14
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadNull.into_u8(),
                Op::Pop.into_u8(),
                Op::LoadInt.into_u8(), 42, 0, 0, 0, 0, 0, 0, 0,
                Op::Pop.into_u8(),
                Op::LoadFloat.into_u8(), 31, 133, 235, 81, 184, 30, 9, 64,
                Op::Return.into_u8(),
            ]
        );
    }

    #[test]
    fn test_build_from_calc_asm() {
        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                LOAD_INT        3
                LOAD_INT        4
                ADD
                LOAD_INT        4
                SUB
                LOAD_INT        2
                MUL
                LOAD_INT        3
                DIV
                LOAD_INT        2
                REM
                LOAD_INT        3
                FLOOR_DIV
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadInt.into_u8(), 4, 0, 0, 0, 0, 0, 0, 0,
                Op::Add.into_u8(),
                Op::LoadInt.into_u8(), 4, 0, 0, 0, 0, 0, 0, 0,
                Op::Sub.into_u8(),
                Op::LoadInt.into_u8(), 2, 0, 0, 0, 0, 0, 0, 0,
                Op::Mul.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Div.into_u8(),
                Op::LoadInt.into_u8(), 2, 0, 0, 0, 0, 0, 0, 0,
                Op::Rem.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::FloorDiv.into_u8(),
                Op::Return.into_u8(),
            ]
        );
    }

    #[test]
    fn test_build_from_compare_asm() {
        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                LOAD_INT        3
                LOAD_INT        4
                EQ
                LOAD_INT        3
                NE
                LOAD_INT        3
                LT
                LOAD_INT        3
                LE
                LOAD_INT        3
                GT
                LOAD_INT        3
                GE
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadInt.into_u8(), 4, 0, 0, 0, 0, 0, 0, 0,
                Op::Eq.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Ne.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Lt.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Le.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Gt.into_u8(),
                Op::LoadInt.into_u8(), 3, 0, 0, 0, 0, 0, 0, 0,
                Op::Ge.into_u8(),
                Op::Return.into_u8(),
            ]
        );
    }

    #[test]
    fn test_build_from_logical_asm() {
        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                LOAD_BOOL       true
                LOAD_BOOL       false
                AND
                LOAD_BOOL       true
                OR
                NOT
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadBool.into_u8(), 1,
                Op::LoadBool.into_u8(), 0,
                Op::And.into_u8(),
                Op::LoadBool.into_u8(), 1,
                Op::Or.into_u8(),
                Op::Not.into_u8(),
                Op::Return.into_u8(),
            ]
        );
    }

    #[test]
    fn test_build_from_label_and_jump_asm() {
        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
              .label.000001:
                LOAD_BOOL       true
                JUMP_IF_TRUE    .label.000001

              .label.000002:
                LOAD_BOOL       false
                JUMP_IF_FALSE   .label.000002

              .label.000003:
                JUMP            .label.000003

                LOAD_NULL
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadBool.into_u8(), 1,
                Op::JumpIfTrue.into_u8(), 0, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadBool.into_u8(), 0,
                Op::JumpIfFalse.into_u8(), 11, 0, 0, 0, 0, 0, 0, 0,
                Op::Jump.into_u8(), 22, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadNull.into_u8(),
                Op::Return.into_u8(),
            ]
        );

        let bytecode = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                LOAD_INT        42
                LOAD_INT        42
                EQ
                JUMP_IF_FALSE   .label.000001
                LOAD_INT        0
                JUMP            .label.000002
              .label.000001:
                LOAD_INT        42
              .label.000002:
                RETURN
            }
        "#}).unwrap();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![
                Op::LoadInt.into_u8(), 42, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadInt.into_u8(), 42, 0, 0, 0, 0, 0, 0, 0,
                Op::Eq.into_u8(),
                Op::JumpIfFalse.into_u8(), 46, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadInt.into_u8(), 0, 0, 0, 0, 0, 0, 0, 0,
                Op::Jump.into_u8(), 55, 0, 0, 0, 0, 0, 0, 0,
                Op::LoadInt.into_u8(), 42, 0, 0, 0, 0, 0, 0, 0,
                Op::Return.into_u8(),
            ]
        );

        let build_result = build_bytecode_from_asm(indoc! {r#"
            ifunc 0 {
                JUMP_IF_TRUE    .label.unknown
                RETURN
            }
        "#});
        match build_result {
            Err(Error::UnknownLabel(label)) => assert_eq!(label, Label::new("label.unknown")),
            // TODO (PeterlitsZo): Wait, I mean, make the `Bytecode` implement `Debug` looks better.
            _ => panic!("unexpected result"),
        }
    }
}
