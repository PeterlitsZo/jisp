use crate::{bc::{Bytecode, Op}, error::Error, value::{Value, ValueKind}};

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
    pub fn run(mut self) -> Result<Value, Error<'static>> {
        let mut locals = vec![Value::Null; self.bytecode.locals() as usize];
        let bytes = self.bytecode.bytes();

        loop {
            let op = Op::from_byte(bytes[self.pc]).unwrap();
            let new_bad_value_type = |a: &Value, b: &Value| {
                Error::runtime(
                    format!(
                        "Bad value type for {} at pc={}: {} and {}",
                        op.display(), self.pc, a.kind().display(), b.kind().display()
                    )
                )
            };

            match op {
                Op::PushInt => {
                    let val = &bytes[self.pc+1..self.pc+9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    self.stack.push(Value::Int(val));
                }
                Op::PushBool => {
                    let val = &bytes[self.pc+1];
                    let val = match *val {
                        0 => false,
                        1 => true,
                        _ => return Err(Error::runtime(
                            format!(
                                "Bad val for {} at pc={}: {}",
                                op.display(), self.pc, *val,
                            )
                        )),
                    };
                    self.stack.push(Value::Bool(val));
                }
                Op::PushNull => {
                    self.stack.push(Value::Null);
                }
                Op::Pop => {
                    self.stack.pop().unwrap();
                }

                Op::Jump => {
                    let idx = &bytes[self.pc+1..self.pc+5];
                    let idx = u32::from_le_bytes(idx.try_into().unwrap());
                    self.pc = idx as usize;
                    continue;
                }
                Op::JumpIfTrue => {
                    let idx = &bytes[self.pc+1..self.pc+5];
                    let idx = u32::from_le_bytes(idx.try_into().unwrap());
                    if self.stack.pop().unwrap().as_bool().unwrap() {
                        self.pc = idx as usize;
                        continue;
                    }
                }
                Op::JumpIfFalse => {
                    let idx = &bytes[self.pc+1..self.pc+5];
                    let idx = u32::from_le_bytes(idx.try_into().unwrap());
                    if !self.stack.pop().unwrap().as_bool().unwrap() {
                        self.pc = idx as usize;
                        continue;
                    }
                }

                Op::Load => {
                    let idx = &bytes[self.pc+1..self.pc+5];
                    let idx = u32::from_le_bytes(idx.try_into().unwrap());
                    self.stack.push(locals[idx as usize].clone());
                }
                Op::Store => {
                    let idx = &bytes[self.pc+1..self.pc+5];
                    let idx = u32::from_le_bytes(idx.try_into().unwrap());
                    locals[idx as usize] = self.stack.pop().unwrap();
                }

                Op::Ret => {
                    return Ok(self.stack.last().unwrap().clone());
                }

                Op::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Int(a.as_int().unwrap() + b.as_int().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
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
                        _ => return Err(new_bad_value_type(&a, &b))
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
                        _ => return Err(new_bad_value_type(&a, &b))
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
                        _ => return Err(new_bad_value_type(&a, &b))
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
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                },

                Op::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() == b.as_int().unwrap())
                        }
                        (ValueKind::Bool, ValueKind::Bool) => {
                            Value::Bool(a.as_bool().unwrap() == b.as_bool().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
                Op::Ne => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() != b.as_int().unwrap())
                        }
                        (ValueKind::Bool, ValueKind::Bool) => {
                            Value::Bool(a.as_bool().unwrap() != b.as_bool().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
                Op::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() < b.as_int().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
                Op::Le => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() <= b.as_int().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
                Op::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() > b.as_int().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
                Op::Ge => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    let result = match (a.kind(), b.kind()) {
                        (ValueKind::Int, ValueKind::Int) => {
                            Value::Bool(a.as_int().unwrap() >= b.as_int().unwrap())
                        }
                        _ => return Err(new_bad_value_type(&a, &b))
                    };
                    self.stack.push(result);
                }
            }
            self.pc += op.op_len();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::{Asm, AsmFn, AsmStat}, bc::BytecodeBuilder};

    use super::*;

    fn test_runner(asm_stats: &[AsmStat], wanted: Value) {
        let asm = Asm::from([
            AsmFn::from(0, asm_stats)
        ]);
        let bc_builder = BytecodeBuilder::new(asm);
        let bc = bc_builder.build();
        let runner = Runner::new(bc);
        let val = runner.run().unwrap();
        assert_eq!(val, wanted);
    }

    #[test]
    fn calc() {
        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::Ret,
        ], Value::Int(1));
    }

    #[test]
    fn compare() {
        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Eq,
            AsmStat::Ret,
        ], Value::Bool(true));

        test_runner(&[
            AsmStat::PushBool { val: true },
            AsmStat::PushBool { val: true },
            AsmStat::Eq,
            AsmStat::Ret,
        ], Value::Bool(true));

        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Ne,
            AsmStat::Ret,
        ], Value::Bool(false));

        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Gt,
            AsmStat::Ret,
        ], Value::Bool(false));

        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Ge,
            AsmStat::Ret,
        ], Value::Bool(true));

        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Lt,
            AsmStat::Ret,
        ], Value::Bool(false));

        test_runner(&[
            AsmStat::PushInt { val: 1 },
            AsmStat::PushInt { val: 1 },
            AsmStat::Le,
            AsmStat::Ret,
        ], Value::Bool(true));
    }
}