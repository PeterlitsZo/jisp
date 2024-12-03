use crate::asm::AsmStat;

#[derive(Debug, PartialEq)]
pub struct AsmFn {
    locals: u32,
    stats: Vec<AsmStat>,
}

impl AsmFn {
    /// Create an empty [AsmFn].
    pub fn new() -> Self {
        Self { locals: 0, stats: vec![] }
    }

    /// Create an [AsmFn] from [AsmStat]s.
    #[cfg(test)]
    pub fn from<T>(locals: u32, stats: T) -> Self where T: Into<Vec<AsmStat>> {
        Self { locals, stats: stats.into() }
    }

    /// Set the locals number.
    pub fn set_locals(&mut self, locals: u32) {
        self.locals = locals;
    }

    /// Get the locals number.
    pub fn locals(&self) -> u32 {
        self.locals
    }

    /// Push an [AsmStat].
    pub fn push_stat(&mut self, stat: AsmStat) {
        self.stats.push(stat);
    }

    /// Return the ref to [AsmStat]s.
    pub fn stats(&self) -> &[AsmStat] {
        &self.stats
    }
}