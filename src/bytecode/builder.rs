//! The builders for [Bytecode] and [IFunc].

use std::mem::swap;

use crate::asm::{Asm, Stat};

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
    pub(crate) fn parse_asm(&mut self, asm: &Asm) -> &mut Self {
        for asm_ifunc in asm.ifuncs() {
            let mut ifunc_builder = IFunc::builder();
            for stat in asm_ifunc.stats() {
                ifunc_builder.push_stat(stat);
            }
            self.bytecode.ifuncs.push(ifunc_builder.build());
        }

        self
    }

    /// Build the [Bytecode] and reset the builder itself.
    pub(crate) fn build(&mut self) -> Bytecode {
        let mut tmp_bytecode = Bytecode::new();
        swap(&mut tmp_bytecode, &mut self.bytecode);
        tmp_bytecode
    }
}

/// The [IFunc] builder.
pub(super) struct IFuncBuilder {
    ifunc: IFunc,
}

impl IFuncBuilder {
    /// Create a new [IFuncBuilder].
    pub(super) fn new() -> Self {
        Self {
            ifunc: IFunc::new(),
        }
    }

    /// Push a [Stat].
    fn push_stat(&mut self, stat: &Stat) -> &mut Self {
        match stat {
            Stat::Return => self.push_op(Op::Return),

            Stat::LoadNull => self.push_op(Op::LoadNull),
            Stat::LoadInt(val) => self.push_op_args(Op::LoadInt, &val.to_le_bytes()),
            Stat::LoadFloat(val) => self.push_op_args(Op::LoadFloat, &val.to_le_bytes()),
            Stat::LoadBool(val) => self.push_op_args(Op::LoadBool, &[if *val { 1 } else { 0 }]),

            Stat::Pop => self.push_op(Op::Pop),

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

            _ => todo!("impl for Stat Not, And and Or")
        }
    }

    /// Push an opcode without arguments.
    fn push_op(&mut self, op: Op) -> &mut Self {
        self.ifunc.code.push(op.into_u8());
        self
    }

    /// Push an opcode with arguments.
    fn push_op_args(&mut self, op: Op, args: &[u8]) -> &mut Self {
        self.ifunc.code.push(op.into_u8());
        self.ifunc.code.extend(args);
        self
    }

    /// Build the [IFunc] and reset the builder itself.
    fn build(&mut self) -> IFunc {
        let mut tmp_ifunc = IFunc::new();
        swap(&mut tmp_ifunc, &mut self.ifunc);
        tmp_ifunc
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::asm::Asm;
    use crate::bytecode::op::Op;

    use super::*;

    #[test]
    fn test_build_from_simple_asm() {
        let asm = Asm::from_program(indoc! {r#"
            ifunc 0 {
                LOAD_NULL
                POP
                LOAD_INT        42
                POP
                LOAD_FLOAT      3.14
                RETURN
            }
        "#});
        let bytecode = Bytecode::builder().parse_asm(&asm).build();

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
        let asm = Asm::from_program(indoc! {r#"
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
        "#});
        let bytecode = Bytecode::builder().parse_asm(&asm).build();

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
        let asm = Asm::from_program(indoc! {r#"
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
        "#});
        let bytecode = Bytecode::builder().parse_asm(&asm).build();

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
}
