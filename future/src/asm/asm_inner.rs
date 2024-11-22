use crate::asm::AsmStat;

#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    stats: Vec<AsmStat>,
}

impl Asm {
    /// Create an empty [Asm].
    pub fn new() -> Self {
        Self { stats: vec![] }
    }

    /// Create an [Asm] from [AsmStat]s.
    #[cfg(test)]
    pub fn from<T>(stats: T) -> Self where T: Into<Vec<AsmStat>> {
        Self { stats: stats.into() }
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