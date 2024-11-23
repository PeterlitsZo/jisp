/// One token of the source.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pos: TokenPos,
    val: TokenVal<'a>,
}

impl<'a> Token<'a> {
    pub fn new(val: TokenVal<'a>, pos: TokenPos) -> Self {
        Self { pos, val }
    }

    pub fn kind(&self) -> TokenKind {
        self.val.kind()
    }

    pub fn val(&self) -> &TokenVal<'a> {
        &self.val
    }

    pub fn pos(&self) -> TokenPos {
        self.pos
    }

    pub fn is_eof(&self) -> bool {
        self.kind() == TokenKind::Eof
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenPos {
    pub lineno: u32,
    pub offset: u32,
    pub length: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenVal<'a> {
    Lparam,
    Rparam,
    Int(i64),
    Name(&'a str),
    Eof,
}

impl<'a> TokenVal<'a> {
    pub fn kind(&self) -> TokenKind {
        match self {
            Self::Lparam => TokenKind::Lparam,
            Self::Rparam => TokenKind::Rparam,
            Self::Int(_) => TokenKind::Int,
            Self::Name(_) => TokenKind::Name,
            Self::Eof => TokenKind::Eof,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(val) => Some(*val),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&'a str> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Lparam,
    Rparam,
    Int,
    Name,
    Eof,
}

impl TokenKind {
    pub fn display(&self) -> &'static str {
        match self {
            Self::Lparam => "LPARAM",
            Self::Rparam => "RPARAM",
            Self::Int => "INT",
            Self::Name => "NAME",
            Self::Eof => "EOF",
        }
    }
}
