use std::{iter::Peekable, str::Chars};

pub struct Tokener<'a> {
    source: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// The '('.
    Lparam,

    /// The ')'.
    Rparam,

    /// The '{'.
    Lbrace,

    /// The '}'.
    Rbrace,

    /// The '['.
    Lsquare,

    /// The ']'.
    Rsquare,

    /// The ':'.
    Colon,

    /// The ','.
    Comma,

    /// The symbol.
    Sym(String),

    /// The string.
    Str(String),

    /// The number (i64).
    I64(i64),
}

impl<'a> Tokener<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { source: source.chars().peekable() }
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
                ')' | ']' | '}' | ' ' | '\t' | '\n' => break,
                ch => {
                    self.source.next();
                    sym.push(ch);
                }
            }
        }
        Some(Token::Sym(sym))
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

    fn next_str(&mut self) -> Option<Token> {
        let mut str = String::new();

        // Consume the left quote.
        let first = self.source.next();
        assert_eq!(first, Some('"'));

        // Consume the content.
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => break,
                Some(c) => *c,
            };
            match peek_char {
                // TODO (@PeterlitsZo): Support character like '\n'...
                '"' => break,
                ch => {
                    self.source.next();
                    str.push(ch);
                }
            }
        }

        // Consume the right quote.
        let last = self.source.next();
        assert_eq!(last, Some('"'));

        Some(Token::Str(str))
    }
}

impl<'a> Iterator for Tokener<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            let peek_char = self.source.peek();
            let peek_char = match peek_char {
                None => return None,
                Some(c) => *c
            };
            let result = match peek_char {
                ' ' | '\t' | '\n' => {
                    self.source.next();
                    continue;
                },
                token @ ( '(' | ')' | '[' | ']' | '{' | '}' | ':' | ',' ) => {
                    self.source.next();
                    let token = match token {
                        '(' => Token::Lparam,
                        ')' => Token::Rparam,
                        '[' => Token::Lsquare,
                        ']' => Token::Rsquare,
                        '{' => Token::Lbrace,
                        '}' => Token::Rbrace,
                        ':' => Token::Colon,
                        ',' => Token::Comma,
                        _ => panic!("uncoverd token"),
                    };
                    Some(token)
                }
                '0'..='9' => self.next_num(),
                '"' => self.next_str(),
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
        let tokener = Tokener::new("()");
        assert_eq!(tokener.collect::<Vec<Token>>(), vec![
            Token::Lparam, Token::Rparam,
        ]);

        let tokener = Tokener::new("(+ 1 2)");
        assert_eq!(tokener.collect::<Vec<Token>>(), vec![
            Token::Lparam,
            Token::Sym("+".to_string()),
            Token::I64(1),
            Token::I64(2),
            Token::Rparam,
        ]);

        let tokener = Tokener::new(r###"
            (let a { "foo": "bar", "bar": 1 })
            (print a)
        "###);
        assert_eq!(tokener.collect::<Vec<Token>>(), vec![
            Token::Lparam,
            Token::Sym("let".to_string()),
            Token::Sym("a".to_string()),
            Token::Lbrace,
            Token::Str("foo".to_string()),
            Token::Colon,
            Token::Str("bar".to_string()),
            Token::Comma,
            Token::Str("bar".to_string()),
            Token::Colon,
            Token::I64(1),
            Token::Rbrace,
            Token::Rparam,
            Token::Lparam,
            Token::Sym("print".to_string()),
            Token::Sym("a".to_string()),
            Token::Rparam,
        ]);

        let tokener = Tokener::new(r###"
            (-> (Image::from_url "http://foobar.com/example.png")
                (.resize { "width": 100, "height": 100 }))
        "###);
        assert_eq!(tokener.collect::<Vec<Token>>(), vec![
            Token::Lparam,
            Token::Sym("->".to_string()),
            Token::Lparam,
            Token::Sym("Image::from_url".to_string()),
            Token::Str("http://foobar.com/example.png".to_string()),
            Token::Rparam,
            Token::Lparam,
            Token::Sym(".resize".to_string()),
            Token::Lbrace,
            Token::Str("width".to_string()),
            Token::Colon,
            Token::I64(100),
            Token::Comma,
            Token::Str("height".to_string()),
            Token::Colon,
            Token::I64(100),
            Token::Rbrace,
            Token::Rparam,
            Token::Rparam,
        ]);

        let tokener = Tokener::new(r###"
            (fn fac [n]
              (if (== n 0)
                1
                (begin
                  (let nxt (fac (- n 1)))
                  (* n nxt))))
            (print (fac 10))
        "###);
        assert_eq!(tokener.collect::<Vec<Token>>(), vec![
            Token::Lparam,
            Token::Sym("fn".to_string()),
            Token::Sym("fac".to_string()),
            Token::Lsquare,
            Token::Sym("n".to_string()),
            Token::Rsquare,
            Token::Lparam,
            Token::Sym("if".to_string()),
            Token::Lparam,
            Token::Sym("==".to_string()),
            Token::Sym("n".to_string()),
            Token::I64(0),
            Token::Rparam,
            Token::I64(1),
            Token::Lparam,
            Token::Sym("begin".to_string()),
            Token::Lparam,
            Token::Sym("let".to_string()),
            Token::Sym("nxt".to_string()),
            Token::Lparam,
            Token::Sym("fac".to_string()),
            Token::Lparam,
            Token::Sym("-".to_string()),
            Token::Sym("n".to_string()),
            Token::I64(1),
            Token::Rparam,
            Token::Rparam,
            Token::Rparam,
            Token::Lparam,
            Token::Sym("*".to_string()),
            Token::Sym("n".to_string()),
            Token::Sym("nxt".to_string()),
            Token::Rparam,
            Token::Rparam,
            Token::Rparam,
            Token::Rparam,
            Token::Lparam,
            Token::Sym("print".to_string()),
            Token::Lparam,
            Token::Sym("fac".to_string()),
            Token::I64(10),
            Token::Rparam,
            Token::Rparam,
        ]);
    }
}