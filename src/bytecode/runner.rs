use super::{ins, Bytecode};

/// The [Bytecode] runner.
pub struct Runner {
    pc: usize,
    stacks: Vec<i64>,
    bytecode: Bytecode,
}

impl Runner {
    /// Build a [Runner].
    pub fn new(bytecode: Bytecode) -> Self {
        Self { pc: 0, stacks: vec![], bytecode }
    }

    /// Run the bytecode as eval those code.
    pub fn run(mut self) -> i64 {
        let bytes = self.bytecode.bytes();
        loop {
            let byte = bytes[self.pc];
            match byte {
                ins::RET => {
                    self.pc += 1;
                    return *self.stacks.last().unwrap();
                },
                ins::PUSH_I64 => {
                    let val = &bytes[self.pc+1..self.pc+9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    self.stacks.push(val);
                    self.pc += 9;
                },
                ins::ADD_I64 => {
                    let second = self.stacks.pop().unwrap();
                    let first = self.stacks.pop().unwrap();
                    self.stacks.push(first + second);
                    self.pc += 1;
                },
                _ => panic!("unsupported byte"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::{Asm, AsmStatement}, bytecode::bytecode_builder::BytecodeBuilder};

    use super::*;

    #[test]
    fn basic() {
        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, 0xff);

        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::AddI64,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, 3);
    }
}