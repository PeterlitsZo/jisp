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

            // The flag to indicate if the op has jumped or not (if jumped, we
            // do not move the pc because it is already moved).
            let mut jumped = false;

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
                    if ifunc.code().len() < frame.pc + Op::LoadInt.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let val = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let val = i64::from_le_bytes(val.try_into().unwrap());
                    frame.stack.push(Value::int(val));
                }
                Op::LoadFloat => {
                    if ifunc.code().len() < frame.pc + Op::LoadFloat.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let val = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let val = f64::from_le_bytes(val.try_into().unwrap());
                    frame.stack.push(Value::float(val));
                }
                Op::LoadBool => {
                    if ifunc.code().len() < frame.pc + Op::LoadBool.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let val = if ifunc.code()[frame.pc + 1] == 0 { false } else { true };
                    frame.stack.push(Value::bool(val));
                }

                Op::Pop => {
                    Self::pop_1(frame)?;
                }

                Op::JumpIfTrue => {
                    // XXX (PeterlitsZo): How about move those code to helper
                    // struct as its methods?  e.g.
                    //
                    // ```rust
                    // let addr = opMan.loadForJumpIfTrue();
                    // ```
                    //
                    // Not sure if it is a good idea.

                    if ifunc.code().len() < frame.pc + Op::JumpIfTrue.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let addr = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let addr = u64::from_le_bytes(addr.try_into().unwrap());
                    let addr = addr as usize;

                    let arg = Self::pop_1(frame)?;
                    let arg = arg.as_bool()
                        .ok_or_else(|| Error::TypeError {
                            op: Op::JumpIfTrue,
                            arg_kinds: ArgKinds::new(vec![arg.kind()])
                        })?;
                    if arg {
                        frame.pc = addr;
                        jumped = true;
                    }
                }
                Op::JumpIfFalse => {
                    if ifunc.code().len() < frame.pc + Op::JumpIfFalse.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let addr = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let addr = u64::from_le_bytes(addr.try_into().unwrap());
                    let addr = addr as usize;

                    let arg = Self::pop_1(frame)?;
                    let arg = arg.as_bool()
                        .ok_or_else(|| Error::TypeError {
                            op: Op::JumpIfFalse,
                            arg_kinds: ArgKinds::new(vec![arg.kind()])
                        })?;
                    if !arg {
                        frame.pc = addr;
                        jumped = true;
                    }
                }
                Op::Jump => {
                    if ifunc.code().len() < frame.pc + Op::Jump.op_len() {
                        return Err(Error::BadOpcode(op));
                    }
                    let addr = &ifunc.code()[frame.pc + 1..frame.pc + 9];
                    let addr = u64::from_le_bytes(addr.try_into().unwrap());
                    let addr = addr as usize;
                    frame.pc = addr;
                    jumped = true;
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

                Op::Eq => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::eq(arg1, arg2)?);
                }
                Op::Ne => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::ne(arg1, arg2)?);
                }
                Op::Lt => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::lt(arg1, arg2)?);
                }
                Op::Le => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::le(arg1, arg2)?);
                }
                Op::Gt => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::gt(arg1, arg2)?);
                }
                Op::Ge => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::ge(arg1, arg2)?);
                }

                Op::Not => {
                    let arg = Self::pop_1(frame)?;
                    frame.stack.push(Self::not(arg)?);
                }
                Op::And => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::and(arg1, arg2)?);
                }
                Op::Or => {
                    let (arg1, arg2) = Self::pop_2(frame)?;
                    frame.stack.push(Self::or(arg1, arg2)?);
                }
            }

            if !jumped {    
                frame.pc += op.op_len();
            }
        }
    }

    /// Pop a value from the stack.
    fn pop_1(frame: &mut Frame) -> Result<Value> {
        if frame.stack.is_empty() {
            return Err(Error::EmptyStack)?;
        }
        Ok(frame.stack.pop().unwrap())
    }

    /// Pop two values from the stack as (2nd element, 1st element).
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

    fn eq(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Null, Value::Null) => Ok(Value::bool(true)),
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 == arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 as f64 == arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 == arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 == arg2)),
            (Value::Bool(arg1), Value::Bool(arg2)) => Ok(Value::bool(arg1 == arg2)),
            (_arg1, _arg2) => Ok(Value::bool(false)),
        }
    }

    fn ne(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Null, Value::Null) => Ok(Value::bool(false)),
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 != arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 as f64 != arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 != arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 != arg2)),
            (Value::Bool(arg1), Value::Bool(arg2)) => Ok(Value::bool(arg1 != arg2)),
            (_arg1, _arg2) => Ok(Value::bool(true)),
        }
    }

    fn lt(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 < arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool((arg1 as f64) < arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 < arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 < arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Lt,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn le(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 <= arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 as f64 <= arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 <= arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 <= arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Le,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn gt(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 > arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 as f64 > arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 > arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 > arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Gt,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn ge(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Int(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 >= arg2)),
            (Value::Int(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 as f64 >= arg2)),
            (Value::Float(arg1), Value::Int(arg2)) => Ok(Value::bool(arg1 >= arg2 as f64)),
            (Value::Float(arg1), Value::Float(arg2)) => Ok(Value::bool(arg1 >= arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Ge,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn not(arg: Value) -> Result<Value> {
        match arg {
            Value::Bool(arg) => Ok(Value::bool(!arg)),
            arg => Err(Error::TypeError {
                op: Op::Not,
                arg_kinds: ArgKinds::new(vec![arg.kind()]),
            }),
        }
    }

    fn and(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Bool(arg1), Value::Bool(arg2)) => Ok(Value::bool(arg1 && arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::And,
                arg_kinds: ArgKinds::new(vec![arg1.kind(), arg2.kind()]),
            }),
        }
    }

    fn or(arg1: Value, arg2: Value) -> Result<Value> {
        match (arg1, arg2) {
            (Value::Bool(arg1), Value::Bool(arg2)) => Ok(Value::bool(arg1 || arg2)),
            (arg1, arg2) => Err(Error::TypeError {
                op: Op::Or,
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
        let bytecode = Bytecode::builder().parse_asm(&asm).unwrap().build();
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

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       true
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));
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

    #[test]
    fn test_simple_compare() {
        let program = indoc! { r#"
            ifunc 0 {
                EQ
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::EmptyStack);

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        1
                EQ

                LOAD_INT        1
                LOAD_INT        2
                EQ
                NOT
                AND

                LOAD_INT        -1
                LOAD_FLOAT      -1.0
                EQ
                AND

                LOAD_NULL
                LOAD_NULL
                EQ
                AND

                LOAD_NULL
                LOAD_INT        1
                EQ
                NOT
                AND

                LOAD_FLOAT      -1.0
                LOAD_FLOAT      -1.0
                EQ
                AND

                LOAD_FLOAT      -1.0
                LOAD_INT        -1
                EQ
                AND

                LOAD_BOOL       true
                LOAD_BOOL       false
                EQ
                NOT
                AND

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        1
                NE

                LOAD_INT        1
                LOAD_INT        2
                NE
                NOT
                OR

                LOAD_INT        -1
                LOAD_FLOAT      -1.0
                NE
                OR

                LOAD_NULL
                LOAD_NULL
                NE
                OR

                LOAD_NULL
                LOAD_INT        1
                NE
                NOT
                OR

                LOAD_FLOAT      -1.0
                LOAD_FLOAT      -1.0
                NE
                OR

                LOAD_FLOAT      -1.0
                LOAD_INT        -1
                NE
                OR

                LOAD_BOOL       true
                LOAD_BOOL       false
                NE
                NOT
                OR

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(false));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        2
                LT

                LOAD_INT        1
                LOAD_INT        1
                LT
                NOT
                AND

                LOAD_INT        2
                LOAD_INT        1
                LT
                NOT
                AND

                LOAD_INT        1
                LOAD_FLOAT      1.0
                LT
                NOT
                AND

                LOAD_FLOAT      1.0
                LOAD_INT        1
                LT
                NOT
                AND

                LOAD_FLOAT      1.0
                LOAD_FLOAT      1.0
                LT
                NOT
                AND

                LOAD_FLOAT      1.0
                LOAD_FLOAT      2.0
                LT
                AND

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_NULL
                LT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Lt,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Null])
        });

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        1
                LE

                LOAD_INT        1
                LOAD_FLOAT      1.0
                LE
                AND

                LOAD_FLOAT      0.1
                LOAD_INT        1
                LE
                AND

                LOAD_FLOAT      1.0
                LOAD_FLOAT      2.0
                LE
                AND

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_NULL
                LE
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Le,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Null])
        });
        
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        1
                GT
                NOT

                LOAD_INT        1
                LOAD_FLOAT      1.0
                GT
                NOT
                AND

                LOAD_FLOAT      0.1
                LOAD_INT        1
                GT
                NOT
                AND

                LOAD_FLOAT      1.0
                LOAD_FLOAT      2.0
                GT
                NOT
                AND

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_NULL
                GT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Gt,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Null])
        });

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        1
                GE

                LOAD_INT        1
                LOAD_FLOAT      1.0
                GE
                AND

                LOAD_FLOAT      0.1
                LOAD_INT        1
                GE
                NOT
                AND

                LOAD_FLOAT      1.0
                LOAD_FLOAT      2.0
                GE
                NOT
                AND

                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_NULL
                GE
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Ge,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Null])
        });
    }

    #[test]
    fn test_simple_logical() {
        let program = indoc! { r#"
            ifunc 0 {
                NOT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::EmptyStack);

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       true
                NOT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(false));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       false
                NOT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       true
                LOAD_BOOL       true
                AND
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       true
                LOAD_BOOL       false
                AND
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(false));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       false
                LOAD_BOOL       true
                OR
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       false
                LOAD_BOOL       false
                OR
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(false));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                NOT
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Not,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int])
        });

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_BOOL       true
                AND
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::And,
            arg_kinds: ArgKinds::new(vec![ValueKind::Int, ValueKind::Bool])
        });

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_INT        1
                OR
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap_err(), Error::TypeError {
            op: Op::Or,
            arg_kinds: ArgKinds::new(vec![ValueKind::Null, ValueKind::Int])
        });
    }

    #[test]
    fn test_label_and_jump() {
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                ADD
                LOAD_FLOAT      3.0
                EQ
                JUMP_IF_FALSE   .label.false
                LOAD_BOOL       true
                JUMP            .label.end
              .label.false:
                LOAD_BOOL       false
              .label.end:
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_FLOAT      2.0
                ADD
                LOAD_FLOAT      3.0
                EQ
                JUMP_IF_TRUE    .label.true
                LOAD_BOOL       false
                JUMP            .label.end
              .label.true:
                LOAD_BOOL       true
              .label.end:
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));

        let program = indoc! { r#"
            ifunc 0 {
              .label.true:
                LOAD_NULL
                JUMP_IF_TRUE    .label.true
                RETURN
            }
        "# };
        // TODO (PeterlitsZo): Support `just fmt` and format the code.
        assert_eq!(
            run_program(program).unwrap_err(),
            Error::TypeError { op: Op::JumpIfTrue, arg_kinds: ArgKinds::new(vec![ValueKind::Null]) }
        );

        let program = indoc! { r#"
            ifunc 0 {
              .label.false:
                LOAD_NULL
                JUMP_IF_FALSE   .label.false
                RETURN
            }
        "# };
        assert_eq!(
            run_program(program).unwrap_err(),
            Error::TypeError { op: Op::JumpIfFalse, arg_kinds: ArgKinds::new(vec![ValueKind::Null]) }
        );

        let program = indoc! { r#"
            ifunc 0 {
                LOAD_BOOL       false
                JUMP_IF_FALSE   .label.true
                LOAD_BOOL       false
                RETURN
              .label.true:
                LOAD_BOOL       true
                RETURN
            }
        "# };
        assert_eq!(run_program(program).unwrap(), Value::bool(true));
    }
}