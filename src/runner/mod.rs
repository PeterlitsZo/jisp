#![allow(dead_code)] // TODO (PeterlitsZo): This module will be used in the future.

use crate::{bytecode::{Bytecode, Op}, error::{Error, Result}, value::Value};

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
        let frame = self.runner.frames.last_mut().unwrap();
        let ifunc = &self.bytecode.ifuncs()[frame.ifunc_idx];

        loop {
            let op = ifunc.code()[frame.pc];
            let op = Op::from_u8(op).ok_or(Error::UnknownOpcode(op))?;

            match op {
                Op::Return => {
                    let value = frame.stack.pop().unwrap();
                    self.runner.frames.pop();
                    return Ok(value);
                }
                Op::LoadNull => {
                    frame.stack.push(Value::null());
                }
                _ => unimplemented!(),
            }
            
            frame.pc += op.op_len();
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::{asm::Asm, bytecode::Bytecode, value::Value};

    use super::Runner;

    #[test]
    fn test_simple_program() {
        let program = indoc! { r#"
            ifunc 0 {
                LOAD_NULL
                RETURN
            }
        "# };
        let asm = Asm::from_program(program);
        let bytecode = Bytecode::builder().parse_asm(&asm).build();
        let mut runner = Runner::new();
        let result = runner.run(&bytecode).unwrap();
        assert_eq!(result, Value::null());
    }
}