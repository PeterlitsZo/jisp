#![allow(dead_code)] // TODO (PeterlitsZo): This module will be used in the future.

//! The ASM module in the Jisp.
//! 
//! Use the [Asm::builder] to create a new [Asm] or use the [Asm::from_program]
//! to build it from ASM program text if you are looking for test.

mod builder;
#[cfg(test)]
mod program;

use builder::{AsmBuilder, IFuncBuilder};

/// Represents the assembly in memory.
/// 
/// It contains i-functions ([IFunc]) to run.
#[derive(Debug, PartialEq)]
pub(super) struct Asm {
    ifuncs: Vec<IFunc>,
}

impl Asm {
    fn new() -> Self {
        Self { ifuncs: vec![] }
    }

    /// Get the [Asm] builder.
    fn builder() -> AsmBuilder {
        AsmBuilder::new()
    }

    /// Build [Asm] from program.  Only for test.
    #[cfg(test)]
    pub(super) fn from_program(program: &str) -> Self {
        use program::Parser;

        Parser::new(program).parse()
    }

    /// Get the i-functions.
    pub(super) fn ifuncs(&self) -> &[IFunc] {
        &self.ifuncs
    }
}

/// The [Asm] i-function.  The `i` means internal.
/// 
/// It contains the statments ([Stat]) to run.
#[derive(Debug, PartialEq)]
pub(super) struct IFunc {
    locals: usize,
    stats: Vec<Stat>,
}

impl IFunc {
    fn new() -> Self {
        Self {
            locals: 0,
            stats: vec![]
        }
    }

    /// Get the [IFunc] builder.
    fn builder() -> IFuncBuilder {
        IFuncBuilder::new()
    }

    /// Get the stats.
    pub(super) fn stats(&self) -> &[Stat] {
        &self.stats
    }
}

/// The label in the [Asm].
/// 
/// XXX (PeterlitsZo): Good idea to make the name `Arc<String>`?
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(super) struct Label {
    name: String,
}

impl Label {
    /// Create a new [Label] by name.
    pub(super) fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    /// Get the name of the label.
    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

/// The [Asm] statement.
#[derive(Debug, PartialEq, Clone)]
pub(super) enum Stat {
    /// The label.  It can be used to jump to.
    Label(Label),

    /// Pop from the ifunc stack top and return the popped value from the
    /// running ifunc.
    Return,

    /// Store the value of the stack top to the local variable (the value will be consumed).
    StoreLocal(u32),
    /// Load the value of the local variable and push it to the ifunc stack top.
    LoadLocal(u32),

    /// Load the null and push it to the ifunc stack top.
    LoadNull,
    /// Load the int and push it to the ifunc stack top.
    LoadInt(i64),
    /// Load the float and push it to the ifunc stack top.
    LoadFloat(f64),
    /// Load the boolean and push it to the ifunc stack top.
    LoadBool(bool),

    /// Drop a value from the ifunc stack top.
    Pop,

    /// Jump to the label if the ifunc stack top is `true` (the value will be consumed).
    JumpIfTrue(Label),
    /// Jump to the label if the ifunc stack top is `false` (the value will be consumed).
    JumpIfFalse(Label),
    /// Jump to the label.
    Jump(Label),

    /// Add two values from the stack top and push the result to the stack top.
    Add,
    /// Subtract two values from the stack top and push the result to the stack top.
    Sub,
    /// Multiply two values from the stack top and push the result to the stack top.
    Mul,
    /// Divide two values from the stack top and push the result to the stack top.
    Div,
    /// Modulo two values from the stack top and push the result to the stack top.
    Rem,
    /// Divide floorly two values from the stack top and push the result to the stack top.
    FloorDiv,

    /// Check if two values from the stack top are equal and push the result to
    /// the stack top.
    Eq,
    /// Check if two values from the stack top are not equal and push the result
    /// to the stack top.
    Ne,
    /// Check if the 2nd value less than the 1st value from the stack top and
    /// push the result to the stack top.
    Lt,
    /// Check if the 2nd value less than or equal with the 1st value from the
    /// stack top and push the result to the stack top.
    Le,
    /// Check if the 2nd value greater than the 1st value from the stack top and
    /// push the result to the stack top.
    Gt,
    /// Check if the 2nd value greater than or equal with the 1st value from the
    /// stack top and push the result to the stack top.
    Ge,

    /// Let the value of the stack top be the opposite.
    Not,
    /// Logical and two values from the stack top and push the result to the
    /// stack top.
    And,
    /// Logical or two values from the stack top and push the result to the
    /// stack top.
    Or,
}
