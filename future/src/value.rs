#[derive(Clone)]
pub enum Value {
    Int(i64),
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Self::Int(val) => format!("{}", val),
        }
    }

    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Int(_) => ValueKind::Int,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(val) => Some(*val),
        }
    }
}

pub enum ValueKind {
    Int,
}