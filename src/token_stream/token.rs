/// One token of the source.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    /// The '('.
    Lparam,

    /// The ')'.
    Rparam,
    
    /// The symbol.
    Sym(String),

    /// The integer of 64-bits.
    I64(i64),
}