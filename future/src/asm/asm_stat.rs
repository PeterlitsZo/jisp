#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsmStat {
    PushInt { val: i64 },
    PushBool { val: bool },
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    Ret,
}