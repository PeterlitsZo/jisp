use std::collections::HashMap;

use super::s::S;

#[derive(Debug, PartialEq, Eq)]
pub enum Item {
    Sym(String),
    Str(String),
    I64(i64),
    Obj(Object),
    Arr(Array),
    S(S),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Object {
    inner: HashMap<String, Item>,
}

impl Object {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn from<T>(value: T) -> Self where T: Into<HashMap<String, Item>> {
        Self { inner: value.into() }
    }

    pub fn push_kv(&mut self, key: String, value: Item) {
        self.inner.insert(key, value);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Array {
    inner: Vec<Item>,
}

impl Array {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn from<T>(value: T) -> Self where T: Into<Vec<Item>> {
        Self { inner: value.into() }
    }

    pub fn push(&mut self, value: Item) {
        self.inner.push(value);
    }
}