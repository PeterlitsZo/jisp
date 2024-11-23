use std::str::CharIndices;

use super::{Token, TokenPos, TokenVal};

/// A stream of [Token]s.
pub struct TokenStream<'a> {
    origin: &'a str,
    source: CharIndices<'a>,
    cur_ch_offset: usize,
    peek_ch_0: Option<char>,
    peek_ch_1_offset: usize,
    peek_ch_1: Option<char>,
    cur_lineno: u32,
    cur_offset: u32,
    eof_pos: TokenPos,
}

impl<'a> TokenStream<'a> {
    /// Create a new [TokenStream] from source.
    pub fn new(source: &'a str) -> Self {
        let mut result = Self {
            origin: source,
            source: source.char_indices(),
            cur_ch_offset: 0,
            peek_ch_0: None,
            peek_ch_1_offset: 0,
            peek_ch_1: None,
            cur_lineno: 1,
            cur_offset: 1,
            eof_pos: TokenPos { lineno: 1, offset: 1, length: 0 }
        };
        result.peek();
        result
    }

    pub fn next(&mut self) -> Token<'a> {
        let cur_ch = match self.peek_ch_0 {
            None => {
                return Token::new(TokenVal::Eof, self.eof_pos);
            }
            Some(c) => c,
        };

        match cur_ch {
            _ if cur_ch.is_ascii_whitespace() => {
                self.consume();

                return self.next();
            },
            token @ ( '(' | ')' ) => {
                let pos = TokenPos {
                    lineno: self.cur_lineno,
                    offset: self.cur_offset,
                    length: 1,
                };
                let token = match token {
                    '(' => Token::new(TokenVal::Lparam, pos),
                    ')' => Token::new(TokenVal::Rparam, pos),
                    _ => panic!("uncoverd token"),
                };
                self.consume();
                self.eof_pos.lineno = pos.lineno;
                self.eof_pos.offset = pos.offset + pos.length;
                token
            },
            '0'..='9' => self.next_num(),
            '-' => {
                if self.peek_ch_1.is_some_and(|c| c.is_ascii_digit()) {
                    self.next_num()
                } else {
                    self.next_name()
                }
            }
            _ => self.next_name(),
        }
    }

    pub fn next_num(&mut self) -> Token<'a> {
        let mut sign = 1;
        let mut num = 0_i64;
        let mut is_first = true;
        let mut pos = TokenPos {
            lineno: self.cur_lineno,
            offset: self.cur_offset,
            length: 0
        };
        loop {
            let cur_ch = match self.peek_ch_0 {
                None => break,
                Some(c) => c,
            };
            match cur_ch {
                '-' if is_first => {
                    self.consume();
                    pos.length += 1;
                    sign = -1;
                }
                c @ '0'..='9' => {
                    self.consume();
                    pos.length += 1;
                    num = num * 10 + (c as i64) - ('0' as i64);
                }
                _ => break,
            }
            if is_first {
                is_first = false;
            }
        }
        let token = Token::new(TokenVal::Int(sign * num), pos);
        self.eof_pos.lineno = pos.lineno;
        self.eof_pos.offset = pos.offset + pos.length;
        token
    }

    pub fn next_name(&mut self) -> Token<'a> {
        let begin_offset = self.cur_ch_offset;
        let mut bytes_cnt = 0;
        let mut pos = TokenPos {
            lineno: self.cur_lineno,
            offset: self.cur_offset,
            length: 0
        };
        loop {
            let cur_ch = match self.peek_ch_0 {
                None => break,
                Some(c) => c,
            };

            match cur_ch {
                _ if cur_ch.is_ascii_whitespace() => break,
                ')' => break,
                _ => {
                    bytes_cnt += cur_ch.len_utf8();
                    self.consume();
                    pos.length += 1;
                }
            }
        }
        let name = &self.origin[begin_offset..(begin_offset + bytes_cnt)];
        let token = Token::new(TokenVal::Name(name), pos);
        self.eof_pos.lineno = pos.lineno;
        self.eof_pos.offset = pos.offset + pos.length;
        token
    }

    /// The origin source code.
    pub fn origin(&self) -> &'a str {
        self.origin
    }

    // Peek some characters if we can.
    fn peek(&mut self) {
        if self.peek_ch_0.is_none() {
            match self.source.next() {
                Some((ch_offset, ch)) => {
                    self.cur_ch_offset = ch_offset;
                    self.peek_ch_0 = Some(ch);
                }
                None => return,
            }
        }
        if self.peek_ch_1.is_none() {
            if let Some((ch_offset, ch)) = self.source.next() {
                self.peek_ch_1_offset = ch_offset;
                self.peek_ch_1 = Some(ch);
            }
        }
    }

    // Consume a character.
    fn consume(&mut self) {
        assert!(self.peek_ch_0.is_some());

        if self.peek_ch_0.unwrap() == '\n' {
            self.cur_lineno += 1;
            self.cur_offset = 1;
        } else {
            self.cur_offset += 1;
        }

        self.peek_ch_0 = None;
        if self.peek_ch_1.is_some() {
            self.peek_ch_0 = self.peek_ch_1;
            self.cur_ch_offset = self.peek_ch_1_offset;
            self.peek_ch_1 = None;
        }

        self.peek();
    }
}

#[cfg(test)]
mod tests {
    use crate::ts::TokenKind;

    use super::*;

    fn test_token_stream(source: &str, tokens: &[(u32, u32, u32, TokenVal)]) {
        let mut token_stream = TokenStream::new(source);
        for (index, token) in tokens.iter().enumerate() {
            let pos = TokenPos { lineno: token.0, offset: token.1, length: token.2 };
            let token = Token::new(token.3.clone(), pos);
            let got_token = token_stream.next();
            assert_eq!(got_token, token, "Check the {} token.", index);
        }
        let got_token = token_stream.next();
        assert_eq!(got_token.kind(), TokenKind::Eof, "Check the last, which should be None.");
    }

    #[test]
    fn basic() {
        test_token_stream("1", &[
            (1, 1, 1, TokenVal::Int(1)),
            (1, 2, 0, TokenVal::Eof),
        ]);

        test_token_stream("-1", &[
            (1, 1, 2, TokenVal::Int(-1)),
            (1, 3, 0, TokenVal::Eof),
        ]);

        test_token_stream("(+ 1 20)\n", &[
            (1, 1, 1, TokenVal::Lparam),
            (1, 2, 1, TokenVal::Name("+")),
            (1, 4, 1, TokenVal::Int(1)),
            (1, 6, 2, TokenVal::Int(20)),
            (1, 8, 1, TokenVal::Rparam),
            (1, 9, 0, TokenVal::Eof),
        ]);
    }

    #[test]
    fn names() {
        test_token_stream("(foo (bar baz) zip)", &[
            (1, 1, 1, TokenVal::Lparam),
            (1, 2, 3, TokenVal::Name("foo")),
            (1, 6, 1, TokenVal::Lparam),
            (1, 7, 3, TokenVal::Name("bar")),
            (1, 11, 3, TokenVal::Name("baz")),
            (1, 14, 1, TokenVal::Rparam),
            (1, 16, 3, TokenVal::Name("zip")),
            (1, 19, 1, TokenVal::Rparam),
            (1, 20, 0, TokenVal::Eof),
        ]);
    }
}