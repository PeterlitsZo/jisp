#[derive(Debug, PartialEq, Eq)]
pub enum AsmStatement {
    Label{ label: AsmLabel },

    Ret,

    PushI64 { val: i64 },
    PushConst { index: u32 },

    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,

    Store { index: u32 }, // Move the top of stack to the local (by index).
    Load { index: u32 }, // Load the local (by index) to the top of stack.

    Jump { label: AsmLabel }, // Jump to the label.
    JumpFalse { label: AsmLabel }, // Jump to the label if false.

    Call { args: u32 },
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct AsmLabel {
    label: String,
}

impl AsmLabel {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self { label: label.into() }
    }
}