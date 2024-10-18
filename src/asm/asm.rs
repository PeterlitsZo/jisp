use super::AsmStatement;

/// The ASM.
#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    statements: Vec<AsmStatement>,
}

impl Asm {
    /// Build a emtpy [Asm].
    pub fn new() -> Self {
        Self { statements: Vec::new() }
    }

    /// Build a non-emtpy [Asm] from statements.
    #[cfg(test)]
    pub fn from<T>(statements: T) -> Self where T: Into<Vec<AsmStatement>> {
        Self { statements: statements.into() }
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