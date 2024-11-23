use std::fmt::Debug;

use crate::ts::TokenPos;

pub enum Error<'a> {
    /// Expected to replaced in the future.
    Todo { msg: String },

    /// Runtime error.
    Runtime { msg: String },

    /// Syntax error.
    Syntax { source: &'a str, pos: TokenPos, msg: String },
}

impl<'a> Error<'a> {
    pub fn todo<T>(msg: T) -> Self where T: Into<String> {
        Self::Todo { msg: msg.into() }
    }

    pub fn runtime<T>(msg: T) -> Self where T: Into<String> {
        Self::Runtime { msg: msg.into() }
    }

    pub fn syntax<T>(source: &'a str, pos: TokenPos, msg: T) -> Self where T: Into<String> {
        Self::Syntax { source, pos, msg: msg.into() }
    }

    fn syntax_fmt(&self) -> String {
        match self {
            Self::Syntax { source, pos, msg } => {
                let mut result = String::new();
                let mut lines = source.lines();

                result.push_str(&format!("{:04} | ", pos.lineno));
                let line = lines.nth(pos.lineno as usize - 1);
                result.push_str(line.unwrap());
                result.push_str("\n     | ");
                result.push_str(&" ".to_string().repeat(pos.offset as usize - 1));
                result.push_str(&"^".to_string().repeat(pos.length.max(1) as usize));
                result.push_str(&format!(" Syntax: {}", msg));
                result
            }
            _ => panic!("unexpected type, wanted Syntax.")
        }
    }
}

impl<'a> Debug for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Todo { msg } => format!("TODO: {}", msg),
            Self::Runtime { msg } => format!("Runtime: {}", msg),
            Self::Syntax { source: _, pos: _, msg: _ } => self.syntax_fmt(),
        })
    }
}