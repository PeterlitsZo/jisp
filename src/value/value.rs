#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Undefined,
    I64(i64),
    Bool(bool),
    Str(String),
    Fn(u32),
}