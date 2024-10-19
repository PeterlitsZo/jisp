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
                token @ ( '(' | ')' ) => {
                    self.skip_char();
                    let token = match token {
                        '(' => Token::new(self.pos, TokenVal::Lparam),
                        ')' => Token::new(self.pos, TokenVal::Rparam),
                        _ => panic!("uncovered token"),
                    };
                    self.pos.offset += 1;
                    self.eof_pos = self.pos;
                    Some(token)
                }
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
    }
}