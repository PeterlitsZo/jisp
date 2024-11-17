use crate::value::{Value, XFn};

use super::AsmStatement;

/// The ASM.
#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    pub consts: Vec<Value>, // The consts.
    pub ifns: Vec<AsmFn>, // The inner functions.
    pub xfns: Vec<XFn>, // The extend functions.
}

impl Asm {
    /// Build an emtpy [Asm].
    pub fn new() -> Self {
        Self {
            consts: Vec::new(),
            ifns: Vec::new(),
            xfns: Vec::new(),
        }
    }

    /// Push a [AsmFn].
    pub fn push_fn(&mut self, f: AsmFn) {
        self.ifns.push(f);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct AsmFn {
    pub locals: u32, // The number of local variables.
    pub statements: Vec<AsmStatement>,
}

impl AsmFn {
    /// Build an empty [AsmFn].
    pub fn new(locals: u32, statements: Vec<AsmStatement>) -> Self {
        Self { locals, statements }
    }

    /// Push a statements.
    pub fn push_statement(&mut self, statement: AsmStatement) {
        self.statements.push(statement);
    }
}