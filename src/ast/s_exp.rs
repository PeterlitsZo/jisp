/// A simple S-expression.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SExp {
    I64(i64),
    Sym(String),
    List(Vec<SExp>),
}