pub enum Value {
    Int(i64),
}

impl Value {
    pub fn display(&self) -> String {
        match self {
            Self::Int(val) => format!("{}", val),
        }
    }
}