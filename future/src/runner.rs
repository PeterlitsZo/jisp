use crate::{bytecode::{Bytecode, Op}, value::{Value, ValueKind}};

pub struct Runner {
    bytecode: Bytecode,
    stack: Vec<Value>,
    pc: usize,
}

impl Runner {
    /// Create a [Runner].
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode, stack: vec![], pc: 0 }
    }

    /// Run and return the [Value].
    pub fn run(mut self) -> Value {
        let bytes = self.bytecode.bytes();
        loop {
            let op = Op::from_byte(bytes[self.pc]).unwrap();
            match op {
                Op::PushInt => {
                    let val = &bytes[self.pc+1..self.pc+9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    self.stack.push(Value::Int(val));
                }
                Op::Pop => {
                    self.stack.pop().unwrap();
                }
                Op::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() + b.as_int().unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Op::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() - b.as_int().unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Op::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() * b.as_int().unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Op::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() / b.as_int().unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Op::Mod => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() % b.as_int().unwrap())
                        }
                    };
                    self.stack.push(result);
                }
                Op::Ret => {
                    return self.stack.last().unwrap().clone();
                }
            }
            self.pc += op.op_len();
        }
    }
}