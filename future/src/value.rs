#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Int(i64),
    Bool(bool),
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Self::Int(val) => format!("{}", val),
            Self::Bool(val) => format!("{}", val),
        }
    }

    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Int(_) => ValueKind::Int,
            Value::Bool(_) => ValueKind::Bool,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(val) => Some(*val),
            _ => None,
        }
    }
}

pub enum ValueKind {
    Int,
    Bool,
}