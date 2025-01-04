#![allow(dead_code)] // TODO (PeterlitsZo): This line need be removed in the future.

//! The bytecode module in the Jisp.
//!
//! The ASM (see [crate::asm::Asm]) will be compiled to the bytecode (see
//! [Bytecode]) and run in the future.  See [Bytecode::builder] to build it.

mod builder;
mod op;

use builder::{BytecodeBuilder, IFuncBuilder};

/// The Jisp bytecode.
struct Bytecode {
    /// The i-functions.
    ifuncs: Vec<IFunc>,
}

impl Bytecode {
    /// Create a empty [Bytecode].
    fn new() -> Self {
        Self { ifuncs: vec![] }
    }

    /// Get the [Bytecode] builder.
    fn builder() -> BytecodeBuilder {
        BytecodeBuilder::new()
    }

    /// Get the i-functions.
    fn ifuncs(&self) -> &[IFunc] {
        &self.ifuncs
    }
}

/// The i-function for the bytecode.  It means the internal-function of Jisp.
///
/// It contains the bytecode to run.
struct IFunc {
    /// The **bytecode** of this i-function.
    code: Vec<u8>,
}

impl IFunc {
    /// Create a empty [IFunc].
    fn new() -> Self {
        Self { code: vec![] }
    }

    /// Get the [IFunc] builder.
    fn builder() -> IFuncBuilder {
        IFuncBuilder::new()
    }

    /// Get the internal bytecode.
    fn code(&self) -> &[u8] {
        &self.code
    }
}
