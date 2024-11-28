#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsmStat {
    PushInt { val: i64 },
    PushBool { val: bool },
    PushNull,
    Pop,

    Load { idx: u32 },
    Store { idx: u32 },

    Ret,

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
}