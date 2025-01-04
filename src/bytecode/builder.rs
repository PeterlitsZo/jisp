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
        }
    }

    /// Push an opcode without arguments.
    fn push_op(&mut self, op: Op) -> &mut Self {
        self.ifunc.code.push(op.into_u8());
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
                RETURN
            }
        "#});
        let bytecode = Bytecode::builder().parse_asm(&asm).build();

        assert_eq!(bytecode.ifuncs.len(), 1);
        assert_eq!(
            bytecode.ifuncs[0].code,
            vec![Op::LoadNull.into_u8(), Op::Return.into_u8()]
        );
    }
}
