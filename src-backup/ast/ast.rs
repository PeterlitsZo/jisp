use super::s::S;

#[derive(Debug, PartialEq, Eq)]
pub struct Ast {
    ss: Vec<S>,
}

impl Ast {
    pub fn new() -> Self {
        return Self { ss: Vec::new() };
    }

    pub fn from<T>(value: T) -> Self where T: Into<Vec<S>> {
        Self { ss: value.into() }
    }

    pub fn push_s(&mut self, s: S) {
        self.ss.push(s);
    }

    pub fn ss(&self) -> impl Iterator<Item=&S> {
        self.ss.iter()
    }
}