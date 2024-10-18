use super::bytecode::{self, Bytecode};

pub struct Runner {
    pc: usize,
    stacks: Vec<i64>,
}

impl Runner {
    pub fn new() -> Self {
        Self { pc: 0, stacks: vec![] }
    }

    pub fn run(&mut self, bytecode: Bytecode) -> i64 {
        loop {
            let byte = bytecode.bytes[self.pc];
            match byte {
                bytecode::RET => {
                    self.pc += 1;
                    return *self.stacks.last().unwrap();
                },
                bytecode::PUSH_I64 => {
                    let val = &bytecode.bytes[self.pc+1..self.pc+9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    self.stacks.push(val);
                    self.pc += 9;
                },
                bytecode::ADD_I64 => {
                    let right = self.stacks.pop().unwrap();
                    let left = self.stacks.pop().unwrap();
                    self.stacks.push(left + right);
                    self.pc += 1;
                },
                bytecode::SUB_I64 => {
                    let right = self.stacks.pop().unwrap();
                    let left = self.stacks.pop().unwrap();
                    self.stacks.push(left - right);
                    self.pc += 1;
                },
                _ => panic!("unsupported byte"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bytecode::BytecodeBuilder;

    use crate::bytecode::asm::{Asm, AsmStatement};

    use super::*;

    fn run(asm: Asm) -> i64 {
        let bytecode_builder = BytecodeBuilder::new(asm);
        let bytecode = bytecode_builder.build();
        let mut runner = Runner::new();
        runner.run(bytecode)
    }

    #[test]
    fn basic() {
        let val = run(Asm::from([
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ]));
        assert_eq!(val, 0xff);

        let val = run(Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::AddI64,
            AsmStatement::Ret,
        ]));
        assert_eq!(val, 3);

        let val = run(Asm::from([
            AsmStatement::PushI64 { val: 10 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::SubI64,
            AsmStatement::Ret,
        ]));
        assert_eq!(val, 5);
    }
}