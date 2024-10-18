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
                '0'..='9' => self.next_num(),
                _ => unimplemented!("peek_char should be in /[ \\t\\n0-9]/ now"),
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
    }
}