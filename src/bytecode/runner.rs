use crate::value::Value;

use super::{bytecode::BytecodeFn, ins, Bytecode};

/// The [Bytecode] runner.
pub struct Runner {
    bytecode: Bytecode,
}

impl Runner {
    /// Build a [Runner].
    pub fn new(bytecode: Bytecode) -> Self {
        Self { bytecode }
    }

    /// Run the bytecode as eval those code.
    pub fn run(self) -> Value {
        self.run_frame(0, vec![])
    }

    fn run_frame(&self, index: usize, args: Vec<Value>) -> Value {
        let frame = RunnerFrame::new(&self, index, args);
        frame.run()
    }
}

pub struct RunnerFrame<'r> {
    runner: &'r Runner,

    func: &'r BytecodeFn, // The function running.
    pc: usize, // The program counter.
    stack: RunnerStack, // The stack.
    locals: RunnerLocals, // The local variables.
}

impl<'r> RunnerFrame<'r> {
    /// Build a [Runner].
    pub fn new(runner: &'r Runner, index: usize, args: Vec<Value>) -> Self {
        let func = &runner.bytecode.ifns[index];
        let mut locals = RunnerLocals::new(func.locals as usize);
        for (i, val) in args.iter().enumerate() {
            locals.set(i, val.clone());
        }

        Self {
            runner,

            func,
            pc: 0,
            stack: RunnerStack::new(),
            locals,
        }
    }

    /// Run the bytecode as eval those code.
    pub fn run(mut self) -> Value {
        let bytes = self.func.bytes();
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
                ins::PUSH_CONST => {
                    let index = &bytes[self.pc+1..self.pc+5];
                    let index = u32::from_le_bytes(index.try_into().unwrap());
                    self.stack.push(self.runner.bytecode.consts[index as usize].clone());
                    self.pc += 5;
                }

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

                ins::LOAD => {
                    let index = &bytes[self.pc+1..self.pc+5];
                    let index = u32::from_le_bytes(index.try_into().unwrap());
                    self.stack.push(self.locals.get(index as usize));
                    self.pc += 5;
                },
                ins::STORE => {
                    let index = &bytes[self.pc+1..self.pc+5];
                    let index = u32::from_le_bytes(index.try_into().unwrap());
                    let val = self.stack.pop();
                    self.locals.set(index as usize, val);
                    self.pc += 5;
                },

                ins::JUMP => {
                    let offset = &bytes[self.pc+1..self.pc+5];
                    let offset = u32::from_le_bytes(offset.try_into().unwrap());
                    self.pc = offset as usize;
                }
                ins::JUMP_FALSE => {
                    let offset = &bytes[self.pc+1..self.pc+5];
                    let offset = u32::from_le_bytes(offset.try_into().unwrap());
                    if !self.stack.pop_bool() {
                        self.pc = offset as usize;
                    } else {
                        self.pc += 5;
                    }
                }

                ins::CALL => {
                    let args = &bytes[self.pc+1..self.pc+5];
                    let args = u32::from_le_bytes(args.try_into().unwrap());

                    let mut arg_values = vec![];
                    for _ in 0..args {
                        arg_values.push(self.stack.pop());
                    }
                    arg_values.reverse();

                    let func = self.stack.pop();
                    let res = match func {
                        Value::IFn(index) => {
                            self.runner.run_frame(index as usize, arg_values)
                        }
                        Value::XFn(index) => {
                            let xfn = &self.runner.bytecode.xfns[index as usize];
                            xfn.call(arg_values)
                        }
                        _ => panic!("runtime error"),
                    };
                    self.stack.push(res);

                    self.pc += 5;
                }

                _ => panic!("unexpected byte"),
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

    fn push(&mut self, val: Value) {
        self.stack.push(val);
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

struct RunnerLocals {
    locals: Vec<Value>
}

impl RunnerLocals {
    fn new(length: usize) -> Self {
        Self { locals: vec![Value::Undefined; length] }
    }

    fn get(&self, index: usize) -> Value {
        self.locals[index].clone()
    }

    fn set(&mut self, index: usize, value: Value) {
        self.locals[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::{Asm, AsmFn, AsmLabel, AsmStatement}, bytecode::{self, bytecode_builder::BytecodeBuilder}, value::XFn};

    use super::*;

    #[test]
    fn calc() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 0xff },
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(0xff));

        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::PushI64 { val: 3 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(6));

        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 6 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Sub,
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Sub,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(3));
    }

    #[test]
    fn compare() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 6 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(false));

        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(true));

        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::PushI64 { val: 255 },
            AsmStatement::Ne,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::Bool(false));
    }

    #[test]
    fn locals() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(2, vec![
            AsmStatement::PushI64 { val: 13 },
            AsmStatement::Store { index: 0 },
            AsmStatement::PushI64 { val: 12 },
            AsmStatement::Store { index: 1 },
            AsmStatement::Load { index: 0 },
            AsmStatement::Load { index: 1 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(25));
    }

    #[test]
    fn label_jump() {
        let mut asm = Asm::new();
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Mul,
            AsmStatement::Label { label: AsmLabel::new(".L2") },
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(2));
    }

    #[test]
    fn function() {
        let mut asm = Asm::new();
        asm.consts.extend_from_slice(&[
            Value::IFn(1),
        ]);
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Call { args: 0 },
            AsmStatement::Ret,
        ]));
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(5));

        let mut asm = Asm::new();
        asm.consts = vec![
            Value::IFn(1),
        ];
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 3 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 2 },
            AsmStatement::Ret,
        ]));
        asm.push_fn(AsmFn::new(2, vec![
            AsmStatement::Load { index: 0 },
            AsmStatement::Load { index: 1 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(8));

        let mut asm = Asm::new();
        asm.consts = vec![
            Value::IFn(1),
        ];
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 1 },
            AsmStatement::Ret,
        ]));
        asm.push_fn(AsmFn::new(1, vec![
            AsmStatement::Load { index: 0 },
            AsmStatement::PushI64 { val: 0 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Load { index: 0 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Sub,
            AsmStatement::Call { args: 1 },
            AsmStatement::Load { index: 0 },
            AsmStatement::Mul,
            AsmStatement::Label { label: AsmLabel::new(".L2") },
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(120));

        let mut asm = Asm::new();
        asm.xfns = vec![
            XFn::new("x_add_3".to_string(), |args: Vec<Value>| {
                assert!(args.len() == 1);
                match args[0] {
                    Value::I64(val) => Value::I64(val + 3),
                    _ => panic!("unexpected value"),
                }
            })
        ];
        asm.consts = vec![
            Value::XFn(0),
        ];
        asm.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 1 },
            AsmStatement::Ret,
        ]));
        let bytecode = BytecodeBuilder::new(asm).build();
        let result = Runner::new(bytecode).run();
        assert_eq!(result, Value::I64(8));
    }
}