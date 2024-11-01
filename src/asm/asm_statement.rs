#[derive(Debug, PartialEq, Eq)]
pub enum AsmStatement {
    Ret,

    PushI64 { val: i64 },

    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}