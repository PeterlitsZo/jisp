use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Value {
    Null,
    Undefined,
    I64(i64),
    Bool(bool),
    Str(String),
    IFn(u32),
    XFn(u32),
}

pub struct XFn {
    id: String,
    inner: Box<dyn Fn(Vec<Value>) -> Value>,
}

impl PartialEq for XFn {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for XFn {}

impl Debug for XFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XFn")
    }
}

impl XFn {
    pub fn new<F>(id: String, f: F) -> Self where F: Fn(Vec<Value>) -> Value + 'static {
        XFn { id, inner: Box::new(f) }
    }

    pub fn call(&self, args: Vec<Value>) -> Value {
        (self.inner)(args)
    }
}