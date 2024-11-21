use crate::{asm::Asm, asm_stat::AsmStat, bytecode::{Bytecode, Op}};

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
        for stat in self.asm.stats() {
            match stat {
                AsmStat::PushInt { val } => {
                    bc.push_byte(Op::PushInt.byte());
                    bc.push_bytes(&val.to_le_bytes());
                },
                AsmStat::Pop => bc.push_byte(Op::Pop.byte()),
                AsmStat::Add => bc.push_byte(Op::Add.byte()),
                AsmStat::Sub => bc.push_byte(Op::Sub.byte()),
                AsmStat::Mul => bc.push_byte(Op::Mul.byte()),
                AsmStat::Div => bc.push_byte(Op::Div.byte()),
                AsmStat::Mod => bc.push_byte(Op::Mod.byte()),
                AsmStat::Ret => bc.push_byte(Op::Ret.byte()),
            }
        }
        bc
    }
}