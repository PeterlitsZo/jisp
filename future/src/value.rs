#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Self::Int(val) => format!("{}", val),
            Self::Float(val) => format!("{:?}", val),
            Self::Bool(val) => format!("{}", val),
            Self::Null => "null".to_string(),
        }
    }

    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Int(..) => ValueKind::Int,
            Value::Float(..) => ValueKind::Float,
            Value::Bool(..) => ValueKind::Bool,
            Value::Null => ValueKind::Null,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(val) => Some(*val),
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
    Float,
    Bool,
    Null,
}

impl ValueKind {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Int => "Int",
            Self::Float => "Float",
            Self::Bool => "Bool",
            Self::Null => "Null",
        }
    }
}