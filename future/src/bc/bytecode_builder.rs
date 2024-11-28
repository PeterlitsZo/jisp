use crate::{asm::{Asm, AsmStat}, bc::{Bytecode, Op}};

pub struct BytecodeBuilder {
    asm: Asm,
}

impl BytecodeBuilder {
    /// Create a [BytecodeBuilder] from [Asm].
    pub fn new(asm: Asm) -> Self {
        Self { asm }
    }

    /// Consume self and return a built [Bytecode].
    pub fn build(self) -> Bytecode {
        let mut bc = Bytecode::new();
        bc.set_locals(self.asm.fns()[0].locals());
        for stat in self.asm.fns()[0].stats() {
            match stat {
                AsmStat::PushInt { val } => {
                    bc.push_byte(Op::PushInt.byte());
                    bc.push_bytes(&val.to_le_bytes());
                },
                AsmStat::PushBool { val } => {
                    bc.push_byte(Op::PushBool.byte());
                    bc.push_byte(if *val { 1 } else { 0 });
                },
                AsmStat::PushNull => bc.push_byte(Op::PushNull.byte()),
                AsmStat::Pop => bc.push_byte(Op::Pop.byte()),

                AsmStat::Load { idx } => {
                    bc.push_byte(Op::Load.byte());
                    bc.push_bytes(&idx.to_le_bytes());
                }
                AsmStat::Store { idx } => {
                    bc.push_byte(Op::Store.byte());
                    bc.push_bytes(&idx.to_le_bytes());
                }

                AsmStat::Ret => bc.push_byte(Op::Ret.byte()),

                AsmStat::Add => bc.push_byte(Op::Add.byte()),
                AsmStat::Sub => bc.push_byte(Op::Sub.byte()),
                AsmStat::Mul => bc.push_byte(Op::Mul.byte()),
                AsmStat::Div => bc.push_byte(Op::Div.byte()),
                AsmStat::Mod => bc.push_byte(Op::Mod.byte()),

                AsmStat::Eq => bc.push_byte(Op::Eq.byte()),
                AsmStat::Ne => bc.push_byte(Op::Ne.byte()),
                AsmStat::Lt => bc.push_byte(Op::Lt.byte()),
                AsmStat::Le => bc.push_byte(Op::Le.byte()),
                AsmStat::Gt => bc.push_byte(Op::Gt.byte()),
                AsmStat::Ge => bc.push_byte(Op::Ge.byte()),
            }
        }
        bc
    }
}