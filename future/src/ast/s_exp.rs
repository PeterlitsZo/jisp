/// A S-expression.
#[derive(Debug, PartialEq, Eq)]
pub enum SExp<'a> {
    Int(i64),
    Name(&'a str),
    List(Vec<SExp<'a>>),
}

impl<'a> SExp<'a> {
    pub fn kind(&self) -> SExpKind {
        match self {
            Self::Int(_) => SExpKind::Int,
            Self::Name(_) => SExpKind::Name,
            Self::List(_) => SExpKind::List,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&str> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[SExp<'a>]> {
        match self {
            Self::List(lst) => Some(lst),
            _ => None,
        }
    }
}

pub enum SExpKind {
    Int,
    Name,
    List,
}