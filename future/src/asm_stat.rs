#[derive(Debug, Clone, Copy)]
pub enum AsmStat {
    PushInt { val: i64 },
    Pop,
    Add,
    Sub,
    Ret,
}