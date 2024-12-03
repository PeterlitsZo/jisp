use super::SExp;

#[derive(Debug, PartialEq)]
pub struct Ast<'a> {
    s_exps: Vec<SExp<'a>>,
}

impl<'a> Ast<'a> {
    /// Create an empty [Ast].
    pub fn new() -> Self {
        Self { s_exps: vec![] }
    }

    /// Push a [SExp].
    pub fn push_s_exp(&mut self, s_exp: SExp<'a>) {
        self.s_exps.push(s_exp);
    }

    /// Create an [Ast] from [SExp] list.
    #[cfg(test)]
    pub fn from<T>(s_exps: T) -> Self where T: Into<Vec<SExp<'a>>> {
        Self { s_exps: s_exps.into() }
    }

    /// Return the ref to the inner [SExp]s.
    pub fn s_exps(&self) -> &[SExp<'a>] {
        &self.s_exps
    }
}