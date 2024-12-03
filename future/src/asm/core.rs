use super::AsmFn;

#[derive(Debug, PartialEq)]
pub struct Asm {
    fns: Vec<AsmFn>,
}

impl Asm {
    /// Create an empty [Asm].
    pub fn new() -> Self {
        Self { fns: vec![] }
    }

    /// Create an [Asm] from [AsmFn]s.
    #[cfg(test)]
    pub fn from<T>(fns: T) -> Self where T: Into<Vec<AsmFn>> {
        Self { fns: fns.into() }
    }

    /// Push a [AsmFn].
    pub fn push_fn(&mut self, func: AsmFn) {
        self.fns.push(func);
    }

    /// Return the reference of functions.
    pub fn fns(&self) -> &[AsmFn] {
        &self.fns
    }
}
