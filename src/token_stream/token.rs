/// One token of the source.
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pos: TokenPos,
    val: TokenVal,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TokenPos {
    pub lineno: u32,
    pub offset: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenVal {
    /// The '('.
    Lparam,

    /// The ')'.
    Rparam,

    /// The '['.
    Lsquare,

    /// The ']'.
    Rsquare,
    
    /// The symbol.
    Sym(String),

    /// The string.
    Str(String),

    /// The integer of 64-bits.
    I64(i64),

    /// The end of file.
    EOF,
}

impl Token {
    pub fn new(pos: TokenPos, val: TokenVal) -> Self {
        Self { pos, val }
    }

    pub fn pos(&self) -> TokenPos {
        self.pos
    }

    pub fn val(&self) -> &TokenVal {
        &self.val
    }
}

impl TokenVal {
    pub fn name (&self) -> &'static str {
        match self {
            TokenVal::Lparam => "LPARAM",
            TokenVal::Rparam => "RPARAM",
            TokenVal::Lsquare => "LSQUARE",
            TokenVal::Rsquare => "RSQUARE",
            TokenVal::I64(_) => "I64",
            TokenVal::Sym(_) => "SYM",
            TokenVal::Str(_) => "STR",
            TokenVal::EOF => "EOF",
        }
    }
}