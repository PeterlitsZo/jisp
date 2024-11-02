use super::AsmStatement;

/// The ASM.
#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    pub locals: u32, // The number of local variables.
    statements: Vec<AsmStatement>,
}

impl Asm {
    /// Build a emtpy [Asm].
    pub fn new(locals: u32) -> Self {
        Self {
            locals,
            statements: Vec::new()
        }
    }

    /// Build a non-emtpy [Asm] from statements.
    #[cfg(test)]
    pub fn from<T>(locals: u32, statements: T) -> Self where T: Into<Vec<AsmStatement>> {
        Self {
            locals,
            statements: statements.into()
        }
    }

    /// Push a statement to the [Asm].
    pub fn push_statement(&mut self, statement: AsmStatement) {
        self.statements.push(statement);
    }

    /// Read the [AsmStatement] one by one.
    pub fn statements(&self) -> impl Iterator<Item = &AsmStatement> {
        self.statements.iter()
    }
}