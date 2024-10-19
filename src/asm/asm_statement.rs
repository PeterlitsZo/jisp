#[derive(Debug, PartialEq, Eq)]
pub enum AsmStatement {
    Ret,
    PushI64 { val: i64 },
    AddI64,
    SubI64,
}