use std::{iter::Peekable, str::Chars};

use super::{Token, TokenPos, TokenVal};

/// A stream of [Token].
pub struct TokenStream<'a> {
    // The source code.
    source: Peekable<Chars<'a>>,

    // The source code, but plain.
    source_plain: &'a str,

    /// The position of current token.
    pos: TokenPos,

    /// The position of the EOF.
    eof_pos: TokenPos,

    /// If the EOF is sent.
    eof_sent: bool,
}

impl<'a> TokenStream<'a> {
    /// Create a new [TokenStream] from the source.
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            source_plain: source,
            pos: TokenPos { lineno: 1, offset: 1 },
            eof_pos: TokenPos { lineno: 1, offset: 1 },
            eof_sent: false,
        }
    }

    pub fn source_plain(&self) -> &str {
        self.source_plain
    }

    fn next_num(&mut self) -> Option<Token> {
        let mut num = 0_i64;
        let mut next_pos = self.pos;
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => break,
                Some(c) => *c,
            };
            match peek_char {
                c @ '0'..='9' => {
                    self.skip_char();
                    next_pos.offset += 1;
                    num = num * 10 + (c as i64) - ('0' as i64);
                }
                _ => break,
            }
        }
        let tok = Token::new(self.pos, TokenVal::I64(num));
        self.pos = next_pos;
        self.eof_pos = self.pos;
        Some(tok)
    }

    // TODO Need a error.
    fn next_str(&mut self) -> Result<Token, ()> {
        let mut str = String::new();
        let mut next_pos = self.pos;

        // Skip the quote character.
        next_pos.offset += 1;
        let first_ch = self.source.next();
        if first_ch != Some('"') {
            return Err(());
        }

        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => return Err(()),
                Some(c) => *c,
            };
            match peek_char {
                '"' => break,
                ch => {
                    self.skip_char();
                    next_pos.offset += 1;
                    str.push(ch);
                }
            }
        }

        // Skip the quote character.
        next_pos.offset += 1;
        let last_ch = self.source.next();
        if last_ch != Some('"') {
            return Err(());
        }

        let tok = Token::new(self.pos, TokenVal::Str(str));
        self.pos = next_pos;
        self.eof_pos = self.pos;
        Ok(tok)
    }

    fn next_sym(&mut self) -> Option<Token> {
        let mut sym = String::new();
        let mut next_pos = self.pos;
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => break,
                Some(c) => *c,
            };
            match peek_char {
                ')' | ' ' | '\t' | '\n' => break,
                ch => {
                    self.skip_char();
                    next_pos.offset += 1;
                    sym.push(ch);
                }
            }
        }
        let tok = Token::new(self.pos, TokenVal::Sym(sym));
        self.pos = next_pos;
        self.eof_pos = self.pos;
        Some(tok)
    }

    fn skip_char(&mut self) {
        self.source.next();
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Peek a char.
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None if !self.eof_sent => {
                    self.eof_sent = true;
                    return Some(Token::new(self.eof_pos, TokenVal::EOF));
                },
                None => {
                    return None;
                },
                Some(c) => *c,
            };

            // Try to build a token from chars.
            let result = match peek_char {
                ' ' | '\t' | '\n' => {
                    self.skip_char();
                    if peek_char == '\n' {
                        self.pos.lineno += 1;
                        self.pos.offset = 1;
                    } else {
                        self.pos.offset += 1;
                    }
                    continue;
                },
                token @ ( '(' | ')' | '[' | ']' ) => {
                    self.skip_char();
                    let token = match token {
                        '(' => Token::new(self.pos, TokenVal::Lparam),
                        ')' => Token::new(self.pos, TokenVal::Rparam),
                        '[' => Token::new(self.pos, TokenVal::Lsquare),
                        ']' => Token::new(self.pos, TokenVal::Rsquare),
                        _ => panic!("uncovered token"),
                    };
                    self.pos.offset += 1;
                    self.eof_pos = self.pos;
                    Some(token)
                },
                '"' => self.next_str().ok(),
                '0'..='9' => self.next_num(),
                _ => self.next_sym(),
            };
            return result;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let token_stream = TokenStream::new("1");
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec![
            Token::new(TokenPos{ lineno: 1, offset: 1 }, TokenVal::I64(1)),
            Token::new(TokenPos{ lineno: 1, offset: 2 }, TokenVal::EOF),
        ]);

        let token_stream = TokenStream::new("(+ 1 2)\n");
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec![
            Token::new(TokenPos{ lineno: 1, offset: 1 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 2 }, TokenVal::Sym("+".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 4 }, TokenVal::I64(1)),
            Token::new(TokenPos{ lineno: 1, offset: 6 }, TokenVal::I64(2)),
            Token::new(TokenPos{ lineno: 1, offset: 7 }, TokenVal::Rparam),
            Token::new(TokenPos{ lineno: 1, offset: 8 }, TokenVal::EOF),
        ]);

        let token_stream = TokenStream::new(
            "(let h \"hello\") (let w \"world\") (if (== 1 1) h w)\n"
        );
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec![
            Token::new(TokenPos{ lineno: 1, offset: 1 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 2 }, TokenVal::Sym("let".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 6 }, TokenVal::Sym("h".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 8 }, TokenVal::Str("hello".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 15 }, TokenVal::Rparam),

            Token::new(TokenPos{ lineno: 1, offset: 17 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 18 }, TokenVal::Sym("let".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 22 }, TokenVal::Sym("w".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 24 }, TokenVal::Str("world".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 31 }, TokenVal::Rparam),

            Token::new(TokenPos{ lineno: 1, offset: 33 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 34 }, TokenVal::Sym("if".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 37 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 38 }, TokenVal::Sym("==".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 41 }, TokenVal::I64(1)),
            Token::new(TokenPos{ lineno: 1, offset: 43 }, TokenVal::I64(1)),
            Token::new(TokenPos{ lineno: 1, offset: 44 }, TokenVal::Rparam),
            Token::new(TokenPos{ lineno: 1, offset: 46 }, TokenVal::Sym("h".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 48 }, TokenVal::Sym("w".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 49 }, TokenVal::Rparam),

            Token::new(TokenPos{ lineno: 1, offset: 50 }, TokenVal::EOF),
        ]);
    }

    #[test]
    fn functions() {
        let token_stream = TokenStream::new(
            "(fn ret5 [] 5) (ret5)"
        );
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec![
            Token::new(TokenPos{ lineno: 1, offset: 1 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 2 }, TokenVal::Sym("fn".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 5 }, TokenVal::Sym("ret5".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 10 }, TokenVal::Lsquare),
            Token::new(TokenPos{ lineno: 1, offset: 11 }, TokenVal::Rsquare),
            Token::new(TokenPos{ lineno: 1, offset: 13 }, TokenVal::I64(5)),
            Token::new(TokenPos{ lineno: 1, offset: 14 }, TokenVal::Rparam),

            Token::new(TokenPos{ lineno: 1, offset: 16 }, TokenVal::Lparam),
            Token::new(TokenPos{ lineno: 1, offset: 17 }, TokenVal::Sym("ret5".to_string())),
            Token::new(TokenPos{ lineno: 1, offset: 21 }, TokenVal::Rparam),

            Token::new(TokenPos{ lineno: 1, offset: 22 }, TokenVal::EOF),
        ]);
    }
}