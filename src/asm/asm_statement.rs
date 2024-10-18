#[derive(Debug, PartialEq, Eq)]
pub enum AsmStatement {
    Ret,
    PushI64 { val: i64 },
}