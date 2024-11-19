use crate::asm_stat::AsmStat;

#[derive(Debug)]
pub struct Asm {
    stats: Vec<AsmStat>,
}

impl Asm {
    /// Create an empty [Asm].
    pub fn new() -> Self {
        Self { stats: vec![] }
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