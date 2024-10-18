use std::{iter::Peekable, str::Chars};

use super::Token;

/// A stream of [Token].
pub struct TokenStream<'a> {
    // The source code.
    source: Peekable<Chars<'a>>,
}

impl<'a> TokenStream<'a> {
    /// Create a new [TokenStream] from the source.
    pub fn new(source: &'a str) -> Self {
        Self { source: source.chars().peekable() }
    }

    fn next_num(&mut self) -> Option<Token> {
        let mut num = 0_i64;
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => break,
                Some(c) => *c,
            };
            match peek_char {
                c @ '0'..='9' => {
                    self.source.next();
                    num = num * 10 + (c as i64) - ('0' as i64);
                }
                _ => break,
            }
        }
        Some(Token::I64(num))
    }

    fn next_sym(&mut self) -> Option<Token> {
        let mut sym = String::new();
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => break,
                Some(c) => *c,
            };
            match peek_char {
                ')' | ' ' | '\t' | '\n' => break,
                ch => {
                    self.source.next();
                    sym.push(ch);
                }
            }
        }
        Some(Token::Sym(sym))
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Peek a char.
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => return None,
                Some(c) => *c,
            };

            // Try to build a token from chars.
            let result = match peek_char {
                ' ' | '\t' | '\n' => {
                    self.source.next();
                    continue;
                },
                token @ ( '(' | ')' ) => {
                    self.source.next();
                    let token = match token {
                        '(' => Token::Lparam,
                        ')' => Token::Rparam,
                        _ => panic!("uncovered token"),
                    };
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
            Token::I64(1)
        ]);

        let token_stream = TokenStream::new("(+ 1 2)");
        assert_eq!(token_stream.collect::<Vec<Token>>(), vec![
            Token::Lparam,
            Token::Sym("+".to_string()),
            Token::I64(1),
            Token::I64(2),
            Token::Rparam,
        ]);
    }
}