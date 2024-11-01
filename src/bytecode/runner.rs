use super::{ins, Bytecode};

/// The [Bytecode] runner.
pub struct Runner {
    pc: usize, // The program counter.
    stack: RunnerStack, // The stack.
    bytecode: Bytecode,
}

impl Runner {
    /// Build a [Runner].
    pub fn new(bytecode: Bytecode) -> Self {
        Self { pc: 0, stack: RunnerStack::new(), bytecode }
    }

    /// Run the bytecode as eval those code.
    pub fn run(mut self) -> Value {
        let bytes = self.bytecode.bytes();
        loop {
            let byte = bytes[self.pc];
            match byte {
                ins::RET => {
                    self.pc += 1;
                    return self.stack.pop();
                },

                ins::PUSH_I64 => {
                    let val = &bytes[self.pc+1..self.pc+9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    self.stack.push_i64(val);
                    self.pc += 9;
                },

                ins::ADD => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_i64(first + second);
                    self.pc += 1;
                },
                ins::SUB => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_i64(first - second);
                    self.pc += 1;
                },
                ins::MUL => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_i64(first * second);
                    self.pc += 1;
                },
                ins::DIV => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_i64(first / second);
                    self.pc += 1;
                },
                ins::EQ => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first == second);
                    self.pc += 1;
                }
                ins::NE => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first != second);
                    self.pc += 1;
                }
                ins::LT => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first < second);
                    self.pc += 1;
                }
                ins::LE => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first <= second);
                    self.pc += 1;
                }
                ins::GT => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first > second);
                    self.pc += 1;
                }
                ins::GE => {
                    let second = self.stack.pop_i64();
                    let first = self.stack.pop_i64();
                    self.stack.push_bool(first >= second);
                    self.pc += 1;
                }

                _ => panic!("unsupported byte"),
            }
        }
    }
}

struct RunnerStack {
    stack: Vec<Value>
}

impl RunnerStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn pop(&mut self) -> Value {
        match self.stack.pop() {
            Some(val) => val,
            _ => panic!("runtime error"),
        }
    }

    fn push_i64(&mut self, val: i64) {
        self.stack.push(Value::I64(val));
    }

    fn pop_i64(&mut self) -> i64 {
        match self.stack.pop() {
            Some(Value::I64(val)) => val,
            _ => panic!("runtime error"),
        }
    }

    fn push_bool(&mut self, val: bool) {
        self.stack.push(Value::Bool(val));
    }

    fn pop_bool(&mut self) -> bool {
        match self.stack.pop() {
            Some(Value::Bool(val)) => val,
            _ => panic!("runtime error"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Value {
    I64(i64),
    Bool(bool),
}

#[cfg(test)]
mod tests {
    use crate::{asm::{Asm, AsmStatement}, bytecode::bytecode_builder::BytecodeBuilder};

    use super::*;

    #[test]
    fn calc() {
        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(0xff));

        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::PushI64 { val: 3 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(6));

        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 6 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Sub,
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Sub,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(3));
    }

    #[test]
    fn compare() {
        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 6 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(false));

        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(true));

        let bytecode = BytecodeBuilder::new(Asm::from([
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::Ne,
            AsmStatement::Ret,
        ])).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(false));

    }
}