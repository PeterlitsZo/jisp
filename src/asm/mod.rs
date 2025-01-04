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
    stats: Vec<Stat>,
}

impl IFunc {
    fn new() -> Self {
        Self { stats: vec![] }
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

/// The [Asm] statement.
#[derive(Debug, PartialEq, Clone)]
pub(super) enum Stat {
    /// Pop from the ifunc stack top and return the popped value from the
    /// running ifunc.
    Return,

    /// Load the null and push it to the ifunc stack top.
    LoadNull,
}
