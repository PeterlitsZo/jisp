//! The ASM program text is a simple language to define [Asm] by simple string.
//! 
//! For example, the following program defines an [Asm] with a single [IFunc]
//! that pushes a null value and returns:
//! 
//! ```
//! ifunc 0 {
//!     PUSH_NULL
//!     RETURN
//! }
//! ```
//! 
//! See [Asm::from_program] also.

use std::{iter::Peekable, str::Chars};

use super::{Asm, Stat};
use super::{IFuncBuilder, AsmBuilder};

enum ParserState {
    /// The initial state.
    Init,

    /// The parser is parsing the i-function body.
    IFuncBody,
}

pub(super) struct Parser<'a> {
    source: &'a str,
    state: ParserState,
    cur_ifunc_cnt: i64,

    asm_builder: AsmBuilder,
    ifunc_builder: IFuncBuilder,
}

impl<'a> Parser<'a> {
    /// Create a new [Parser].
    pub(super) fn new(source: &'a str) -> Self {
        Self {
            source,
            state: ParserState::Init,
            cur_ifunc_cnt: 0,

            asm_builder: AsmBuilder::new(),
            ifunc_builder: IFuncBuilder::new(),
        }
    }

    /// Parse the source and return the [Asm].
    /// 
    /// # Panic
    /// 
    /// Panic if the source is invalid.  It is OK because the method is only
    /// called for testing.
    pub(super) fn parse(&mut self) -> Asm {
        // Parse the source line by line.
        for line in self.source.lines() {
            // Get tokens.
            let tokenizer = Tokenizer::new(line);
            let tokens = tokenizer.collect::<Vec<Token>>();

            // Ignore empty line.
            if tokens.is_empty() {
                continue;
            }

            // Step the states.
            match self.state {
                ParserState::Init => self.parse_init(&tokens),
                ParserState::IFuncBody => self.parse_ifunc_body(&tokens),
            }
        }

        // Return the built [Asm].
        self.asm_builder.build()
    }

    fn parse_init(&mut self, tokens: &[Token]) {
        /// Defined which part we are parsing.
        /// 
        /// Now we only have the i-function part.
        enum Part {
            IFunc,
        }

        // Get the part we are parsing.
        let name = tokens[0].as_name().unwrap();
        let part = match name.as_str() {
            "ifunc" => Part::IFunc,
            _ => panic!("unexpected part name: {:?}", name),
        };

        match part {
            // If the part is defined as i-function.  This line should be like
            // `ifunc 0 {`.
            Part::IFunc => {
                assert_eq!(tokens.len(), 3);
                assert_eq!(tokens[1], Token::Int(self.cur_ifunc_cnt));
                assert_eq!(tokens[2], Token::LBRACE);
                self.state = ParserState::IFuncBody;
            }
        }
    }

    fn parse_ifunc_body(&mut self, tokens: &[Token]) {
        // If the line is `}`.  It means the end of the i-function.
        if tokens.len() == 1 && tokens[0] == Token::RBRACE {
            self.cur_ifunc_cnt += 1;
            self.state = ParserState::Init;
            self.asm_builder.push_ifunc(self.ifunc_builder.build());
            return;
        }

        // Parse the line as a [Stat] and push it to the building i-function.
        let op = match tokens[0] {
            Token::Name(ref name) => name,
            _ => panic!("unexpected token: {:?}", tokens[0]),
        };
        match op.as_str() {
            "RETURN" => self.ifunc_builder.push_stat(Stat::Return),

            "LOAD_NULL" => self.ifunc_builder.push_stat(Stat::LoadNull),
            "LOAD_INT" => {
                let val = tokens[1].as_int().unwrap();
                self.ifunc_builder.push_stat(Stat::LoadInt(val))
            }
            "LOAD_FLOAT" => {
                let val = tokens[1].as_float().unwrap();
                self.ifunc_builder.push_stat(Stat::LoadFloat(val))
            }
            "LOAD_BOOL" => {
                let val = tokens[1].as_name().unwrap();
                let val = match val.as_str() {
                    "true" => true,
                    "false" => false,
                    _ => panic!("unexpected bool value: {:?}", val),
                    
                };
                self.ifunc_builder.push_stat(Stat::LoadBool(val))
            }

            "POP" => self.ifunc_builder.push_stat(Stat::Pop),

            "ADD" => self.ifunc_builder.push_stat(Stat::Add),
            "SUB" => self.ifunc_builder.push_stat(Stat::Sub),
            "MUL" => self.ifunc_builder.push_stat(Stat::Mul),
            "DIV" => self.ifunc_builder.push_stat(Stat::Div),
            "REM" => self.ifunc_builder.push_stat(Stat::Rem),
            "FLOOR_DIV" => self.ifunc_builder.push_stat(Stat::FloorDiv),

            "EQ" => self.ifunc_builder.push_stat(Stat::Eq),
            "NE" => self.ifunc_builder.push_stat(Stat::Ne),
            "LT" => self.ifunc_builder.push_stat(Stat::Lt),
            "LE" => self.ifunc_builder.push_stat(Stat::Le),
            "GT" => self.ifunc_builder.push_stat(Stat::Gt),
            "GE" => self.ifunc_builder.push_stat(Stat::Ge),

            _ => panic!("unexpected op: {:?}", op),
        };
    }
}

/// The token of the ASM program.
#[derive(Debug, PartialEq)]
enum Token {
    /// The name token.
    Name(String),

    /// The integer token.
    Int(i64),

    /// The float token.
    Float(f64),

    /// The `{` token.
    LBRACE,

    /// The `{` token.
    RBRACE,
}

impl Token {
    fn as_name(&self) -> Option<&String> {
        match self {
            Self::Name(name) => Some(name),
            _ => None,
        }
    }

    fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(val) => Some(*val),
            _ => None,
        }
    }

    fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(val) => Some(*val),
            _ => None,
        }
    }
}

/// The tokenizer for the ASM program.  The [Token] is produced by this.  It is
/// iterator.
struct Tokenizer<'a> {
    source: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
        }
    }

    fn next_number(&mut self) -> Token {
        enum Mode { Int, Float }

        let mut buffer = String::new();
        let mut mode = Mode::Int;

        if let Some('-') = self.source.peek() {
            buffer.push('-');
            self.source.next();
        }

        while let Some(c) = self.source.peek() {
            if c.is_digit(10) {
                buffer.push(*c);
                self.source.next().unwrap();
            } else {
                break;
            }
        }

        if let Some('.') = self.source.peek() {
            buffer.push('.');
            self.source.next();

            while let Some(c) = self.source.peek() {
                if c.is_digit(10) {
                    buffer.push(*c);
                    self.source.next().unwrap();
                } else {
                    break;
                }
            }

            mode = Mode::Float;
        }

        match mode {
            Mode::Int => {
                let number = buffer.parse::<i64>().unwrap();
                Token::Int(number)
            }
            Mode::Float => {
                let number = buffer.parse::<f64>().unwrap();
                Token::Float(number)
            }
        }
    }

    fn next_name(&mut self) -> Token {
        let mut name = String::new();
        while let Some(c) = self.source.peek() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '.' => {
                    name.push(*c);
                    self.source.next().unwrap();
                }
                _ => break,
            }
        }
        Token::Name(name)
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let ch = self.source.peek();
            let ch = match ch {
                None => return None,
                Some(c) => *c,
            };

            match ch {
                _ if ch.is_ascii_whitespace() => {
                    self.source.next();
                    continue;
                }
                '-' | '0'..='9' => {
                    return Some(self.next_number());
                }
                '{' => {
                    self.source.next();
                    return Some(Token::LBRACE);
                }
                '}' => {
                    self.source.next();
                    return Some(Token::RBRACE);
                }
                'a'..='z' | 'A'..='Z' | '_' | '.' => {
                    return Some(self.next_name());
                }
                _ => panic!("unexpected character: {}", ch),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use crate::asm::Stat;

    use super::*;

    #[test]
    fn test_simple() {
        let mut parser = Parser::new(indoc! {r#"
            ifunc 0 {
                LOAD_NULL
                LOAD_INT        42
                LOAD_FLOAT      3.14
                LOAD_BOOL       true
                LOAD_BOOL       false
                POP
                RETURN
            }
        "#});
        let asm = parser.parse();

        let wanted = Asm::builder()
            .push_ifunc_by(|mut ifunc_builder| {
                ifunc_builder
                    .push_stat(Stat::LoadNull)
                    .push_stat(Stat::LoadInt(42))
                    .push_stat(Stat::LoadFloat(3.14))
                    .push_stat(Stat::LoadBool(true))
                    .push_stat(Stat::LoadBool(false))
                    .push_stat(Stat::Pop)
                    .push_stat(Stat::Return)
                    .build()
            })
            .build();

        assert_eq!(asm, wanted);
    }

    #[test]
    fn test_calc() {
        let mut parser = Parser::new(indoc! {r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        2
                ADD
                LOAD_INT        3
                LOAD_INT        4
                SUB
                LOAD_INT        5
                MUL
                LOAD_FLOAT      1.0
                DIV
                LOAD_INT        6
                REM
                LOAD_INT        7
                FLOOR_DIV
                RETURN
            }
        "#});
        let asm = parser.parse();

        let wanted = Asm::builder()
            .push_ifunc_by(|mut ifunc_builder| {
                ifunc_builder
                    .push_stat(Stat::LoadInt(1))
                    .push_stat(Stat::LoadInt(2))
                    .push_stat(Stat::Add)
                    .push_stat(Stat::LoadInt(3))
                    .push_stat(Stat::LoadInt(4))
                    .push_stat(Stat::Sub)
                    .push_stat(Stat::LoadInt(5))
                    .push_stat(Stat::Mul)
                    .push_stat(Stat::LoadFloat(1.0))
                    .push_stat(Stat::Div)
                    .push_stat(Stat::LoadInt(6))
                    .push_stat(Stat::Rem)
                    .push_stat(Stat::LoadInt(7))
                    .push_stat(Stat::FloorDiv)
                    .push_stat(Stat::Return)
                    .build()
            })
            .build();

        assert_eq!(asm, wanted);
    }

    #[test]
    fn test_compare() {
        let mut parser = Parser::new(indoc! {r#"
            ifunc 0 {
                LOAD_INT        -1
                LOAD_INT        -2
                LT
                LOAD_INT        3
                LOAD_INT        4
                LE
                NE

                RETURN
            }
        "#});
        let asm = parser.parse();

        let wanted = Asm::builder()
            .push_ifunc_by(|mut ifunc_builder| {
                ifunc_builder
                    .push_stat(Stat::LoadInt(-1))
                    .push_stat(Stat::LoadInt(-2))
                    .push_stat(Stat::Lt)
                    .push_stat(Stat::LoadInt(3))
                    .push_stat(Stat::LoadInt(4))
                    .push_stat(Stat::Le)
                    .push_stat(Stat::Ne)
                    .push_stat(Stat::Return)
                    .build()
            })
            .build();

        assert_eq!(asm, wanted);

        let mut parser = Parser::new(indoc! {r#"
            ifunc 0 {
                LOAD_INT        1
                LOAD_INT        2
                GT
                LOAD_INT        3
                LOAD_INT        4
                GE
                EQ

                RETURN
            }
        "#});
        let asm = parser.parse();

        let wanted = Asm::builder()
            .push_ifunc_by(|mut ifunc_builder| {
                ifunc_builder
                    .push_stat(Stat::LoadInt(1))
                    .push_stat(Stat::LoadInt(2))
                    .push_stat(Stat::Gt)
                    .push_stat(Stat::LoadInt(3))
                    .push_stat(Stat::LoadInt(4))
                    .push_stat(Stat::Ge)
                    .push_stat(Stat::Eq)
                    .push_stat(Stat::Return)
                    .build()
            })
            .build();

        assert_eq!(asm, wanted);

    }
}
