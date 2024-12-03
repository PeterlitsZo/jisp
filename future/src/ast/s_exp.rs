/// A S-expression.
#[derive(Debug, PartialEq)]
pub enum SExp<'a> {
    Int(i64),
    Float(f64),
    Name(&'a str),
    List(Vec<SExp<'a>>),
}

impl<'a> SExp<'a> {
    pub fn kind(&self) -> SExpKind {
        match self {
            Self::Int(_) => SExpKind::Int,
            Self::Float(_) => SExpKind::Float,
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

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&'a str> {
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
    Float,
    Name,
    List,
}

impl SExpKind {
    pub fn display(&self) -> &'static str {
        match self {
            SExpKind::Int => "INT",
            SExpKind::Float => "FLOAT",
            SExpKind::Name => "NAME",
            SExpKind::List => "LIST",
        }
    }
}