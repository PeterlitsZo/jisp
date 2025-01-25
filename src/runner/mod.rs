#![allow(dead_code)] // TODO (PeterlitsZo): This module will be used in the future.

use crate::{bytecode::{Bytecode, Op}, error::{ArgKinds, Error, Result}, value::Value};

/// The Jisp runner, which can run the Jisp bytecode.
struct Runner {
    frames: Vec<Frame>,
}

impl Runner {
    /// Create a initial [Runner].
    fn new() -> Self {
        Self { frames: vec![] }
    }

    /// Run the [Bytecode].
    fn run(&mut self, bytecode: &Bytecode) -> Result<Value> {
        // Check the bytecode is empty or not.
        let ifuncs = bytecode.ifuncs();
        if ifuncs.is_empty() {
            return Err(Error::EmptyBytecode);
        }

        // Build the frame and push it.
        let frame = Frame {
            ifunc_idx: 0,
            stack: vec![],
            pc: 0,
        };
        self.frames.push(frame);

        // Run the bytecode from the first frame.
        FrameRunner::new(self, bytecode).run()
    }
}

/// The frame in the runner (see [Runner]), means a function-call frame in the Jisp.
struct Frame {
    /// The index of the i-function.
    ifunc_idx: usize,
    /// The stack.
    stack: Vec<Value>,
    /// The program counter.
    pc: usize,
}

/// The frame (see [Frame]) runner.
struct FrameRunner<'r, 'b> {
    runner: &'r mut Runner,
    bytecode: &'b Bytecode,
}

impl<'r, 'b> FrameRunner<'r, 'b> {
    /// Create a new [FrameRunner].
    fn new(runner: &'r mut Runner, bytecode: &'b Bytecode) -> Self {
        Self { runner, bytecode }
    }

    /// Run the last frame in runner.
    fn run(&mut self) -> Result<Value> {
        self.run_frame()
    }

    fn run_frame(&mut self) -> Result<Value> {
        let frame = self.runner.frames.last_mut().ok_or(Error::NoFrameToRun)?;
        let ifunc = &self.bytecode.ifuncs()[frame.ifunc_idx];

        loop {
            let op = ifunc.code()[frame.pc];
            let op = Op::from_u8(op).ok_or(Error::UnknownOpcode(op))?;

            match op {
                Op::Return => {
                    let value = Self::pop_1(frame)?;
                    self.runner.frames.pop();
                    return Ok(value);
                }

                Op::LoadNull => {
                    frame.stack.push(Value::null());
                }
                Op::LoadInt => {
                    if ifunc.code().len() < frame.pc + 9 {
                        return Err(Error::BadOpcode(op));
                    }
                    let val = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    frame.stack.push(Value::int(val));
                }
                Op::LoadFloat => {
                    if ifunc.code().len() < frame.pc + 9 {
                        return Err(Error::BadOpcode(op));
                    }
                    let val = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let val = f64::from_le_bytes(val.try_into().unwrap());
                    frame.stack.push(Value::float(val));
                }

                Op::Pop => {
                    Self::pop_1(frame)?;
                }

                Op::Add => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::add(arg1, arg2)?);
                }
                Op::Sub => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::sub(arg1, arg2)?);
                }
                Op::Mul => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::mul(arg1, arg2)?);
                }
                Op::Div => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::div(arg1, arg2)?);
                }
                Op::Rem => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::rem(arg1, arg2)?);
                }
                Op::FloorDiv => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::floor_div(arg1, arg2)?);
                }
            }
            
            frame.pc += op.op_len();
        }
    }

    fn pop_1(frame: &mut Frame) -> Result<Value> {
        if frame.stack.is_empty() {
            return Err(Error::EmptyStack)?;
        }
        Ok(frame.stack.pop().unwrap())
    }

    fn pop_2(frame: &mut Frame) -> Result<(Value, Value)> {
        if frame.stack.len() < 2 {
            return Err(Error::EmptyStack)?;
        }
        let arg2 = frame.stack.pop().unwrap();
        let arg1 = frame.stack.pop().unwrap();
        Ok((arg1, arg2))
    }

    fn add(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Int(arg1 + arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 as f64 + arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 + arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 + arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Add,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn sub(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Int(arg1 - arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 as f64 - arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 - arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 - arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Sub,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn mul(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Int(arg1 * arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 as f64 * arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 * arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 * arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Mul,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn div(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 as f64 / arg2 as f64)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 as f64 / arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 / arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 / arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Div,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn rem(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Int(arg1 % arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 as f64 % arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float(arg1 % arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float(arg1 % arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Rem,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn floor_div(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::Int(arg1 / arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::Float((arg1 as f64 / arg2).floor())),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::Float((arg1 / arg2 as f64).floor())),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::Float((arg1 / arg2).floor())),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::FloorDiv,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{asm::Asm, bytecode::Bytecode, error::{ArgKinds, Error, Result}, runner::Op, value::{Value, ValueKind}};

    use super::Runner;

    fn run_program(program: &str) -> Result<Value> {
        let asm = Asm::from_program(program);
        let bytecode = Bytecode::builder().parse_asm(&asm).build();
        let mut runner = Runner::new();
        runner.run(&bytecode)
    }
    
    #[test]
    fn test_simple_program() {
        let program = "";
        assert_eq!(run_program(program).unwrap_err(), Error::EmptyBytecode);

        let program = indoc! { r#"
            ifunc 0 {
                POP
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::EmptyStack);

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                POP
                LOAD_INT        42
                POP
                LOAD_FLOAT      3.14
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(3.14));
    }

    #[test]
    fn test_simple_calc() {
        // Make sure there is enough operands for the op.
        let program = indoc! { r#"
            ifunc 0 {
                ADD
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::EmptyStack);

        // Test op Add, Sub, Mul, Div, Rem and FloorDiv all work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        3
                LOAD_INT        4
                ADD
                LOAD_INT        4
                SUB
                LOAD_INT        4
                MUL
                LOAD_INT        3
                DIV
                LOAD_INT        3
                REM
                LOAD_FLOAT      4.0
                ADD
                LOAD_INT        3
                FLOOR_DIV
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(1.0));

        // Test op Add work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                ADD
                LOAD_FLOAT      3.0
                ADD
                LOAD_INT        4
                ADD
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(10.0));

        // Test the TypeError if there is unsupported operand for op Add.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                ADD
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Add,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });

        // Test op Sub can work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                SUB
                LOAD_FLOAT      3.0
                SUB
                LOAD_INT        4
                SUB
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(-8.0));

        // Test the TypeError if there is unsupported operand for op Sub.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                SUB
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Sub,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });

        // Test op Mul can work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                MUL
                LOAD_FLOAT      3.0
                MUL
                LOAD_INT        4
                MUL
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(24.0));

        // Test the TypeError if there is unsupported operand for op Mul.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                MUL
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Mul,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });

        // Test op Div can work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                DIV
                LOAD_FLOAT      3.0
                DIV
                LOAD_INT        4
                DIV
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(1.0 / 24.0));

        // Test the TypeError if there is unsupported operand for op Mul.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                DIV
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Div,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });

        // Test op Rem can work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        31
                LOAD_INT        15
                REM
                LOAD_INT        31
                LOAD_FLOAT      15.0
                REM
                LOAD_FLOAT      31.0
                LOAD_FLOAT      15.0
                REM
                MUL
                MUL
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(1.0));

        // Test the TypeError if there is unsupported operand for op Mul.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                REM
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Rem,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });

        // Test op FloorDiv can work well.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        31
                LOAD_INT        15
                FLOOR_DIV
                LOAD_INT        31
                LOAD_FLOAT      15.0
                FLOOR_DIV
                LOAD_FLOAT      31.0
                LOAD_FLOAT      15.0
                FLOOR_DIV
                MUL
                MUL
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::float(8.0));

        // Test the TypeError if there is unsupported operand for op Mul.
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_NULL
                FLOOR_DIV
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::FloorDiv,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Null])
        });
    }
}