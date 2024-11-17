pub enum Value {
    Int(i64),
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Self::Int(val) => format!("{}", val),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(num) => Some(*num),
        }
    }
}