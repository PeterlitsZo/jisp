use super::SExp;

/// The AST.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ast {
    s_exps: Vec<SExp>,
}

impl Ast {
    /// Build a empty [Ast].
    pub fn new() -> Self {
        Self { s_exps: Vec::new() }
    }

    /// Build a non-empty [Ast] from value.
    #[cfg(test)]
    pub fn from<T>(value: T) -> Self where T: Into<Vec<SExp>> {
        Self { s_exps: value.into() }
    }

    /// Push a [SExp] to the [Ast].
    pub fn push_s_exp(&mut self, s_exp: SExp) {
        self.s_exps.push(s_exp);
    }

    /// Read the [SExp] one by one.
    pub fn s_exps(&self) -> impl Iterator<Item = &SExp> {
        self.s_exps.iter()
    }
}