use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub enum AsmStat {
    PushInt { val: i64 },
    PushFloat { val: f64 },
    PushBool { val: bool },
    PushNull,
    Pop,

    Label { label: Label },
    Jump { label: Label },
    #[allow(dead_code)] // TODO (peterlitszo): Not used yet.  Remove this line when used.
    JumpIfTrue { label: Label },
    JumpIfFalse { label: Label },

    Load { idx: u32 },
    Store { idx: u32 },

    Ret,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}

impl AsmStat {
    pub fn kind(&self) -> AsmStatKind {
        match self {
            Self::PushInt { .. } => AsmStatKind::PushInt,
            Self::PushFloat { .. } => AsmStatKind::PushFloat,
            Self::PushBool { .. } => AsmStatKind::PushBool,
            Self::PushNull { .. } => AsmStatKind::PushNull,
            Self::Pop => AsmStatKind::Pop,
            Self::Label { .. } => AsmStatKind::Label,
            Self::Jump { .. } => AsmStatKind::Jump,
            Self::JumpIfTrue { .. } => AsmStatKind::JumpIfTrue,
            Self::JumpIfFalse { .. } => AsmStatKind::JumpIfFalse,
            Self::Load { .. } => AsmStatKind::Load,
            Self::Store { .. } => AsmStatKind::Store,
            Self::Ret => AsmStatKind::Ret,
            Self::Add => AsmStatKind::Add,
            Self::Sub => AsmStatKind::Sub,
            Self::Mul => AsmStatKind::Mul,
            Self::Div => AsmStatKind::Div,
            Self::Mod => AsmStatKind::Mod,
            Self::Eq => AsmStatKind::Eq,
            Self::Ne => AsmStatKind::Ne,
            Self::Gt => AsmStatKind::Gt,
            Self::Ge => AsmStatKind::Ge,
            Self::Lt => AsmStatKind::Lt,
            Self::Le => AsmStatKind::Le,
        }
    }

    pub fn as_label(&self) -> Option<&Label> {
        match self {
            Self::Label { label } => Some(label),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub struct Label {
    name: Rc<String>,
}

impl Label {
    pub fn new(name: String) -> Self {
        Self { name: Rc::new(name) }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AsmStatKind {
    PushInt,
    PushFloat,
    PushBool,
    PushNull,
    Pop,

    Label,
    Jump,
    JumpIfTrue,
    JumpIfFalse,

    Load,
    Store,

    Ret,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
}