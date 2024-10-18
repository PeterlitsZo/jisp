use std::iter::Peekable;

use super::{ast::Ast, item::{Array, Item, Object}, s::S, tokener::{Token, Tokener}};

pub struct Builder<'a> {
    tokener: Peekable<Tokener<'a>>,
}

impl<'a> Builder<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { tokener: Tokener::new(source).peekable() }
    }

    pub fn build(&mut self) -> Result<Ast, BuildError> {
        let mut ast = Ast::new();
        loop {
            match self.next_s() {
                Ok(Some(s)) => ast.push_s(s),
                Ok(None) => break,
                Err(err) => return Err(err.wrap("next s"))
            }
        }
        Ok(ast)
    }

    fn next_s(&mut self) -> Result<Option<S>, BuildError> {
        let mut s = S::new();
        let first_token = self.tokener.next();
        match first_token {
            Some(Token::Lparam) => {
                // Time to build the S.

                loop {
                    let next_token = self.tokener.peek();
                    match next_token {
                        Some(Token::Lparam) => {
                            match self.next_s() {
                                Ok(Some(inner_s)) => s.push_item(Item::S(inner_s)),
                                Err(err) => return Err(err.wrap(&format!("try get inner S because of {:?}", Token::Lbrace))),
                                next_s @ Ok(None) => panic!("next s should not return {:?}", next_s),
                            }
                        },
                        Some(Token::Sym(_) | Token::Str(_) | Token::I64(_)) => {
                            let next_token = self.tokener.next();
                            match next_token {
                                Some(Token::Sym(sym)) => {
                                    s.push_item(Item::Sym(sym));
                                },
                                Some(Token::Str(string)) => {
                                    s.push_item(Item::Str(string));
                                },
                                Some(Token::I64(i64)) => {
                                    s.push_item(Item::I64(i64));
                                }
                                _ => panic!("next token must be sym or i64"),
                            }
                        },
                        Some(Token::Lbrace) => {
                            let obj = self.next_obj();
                            match obj {
                                Ok(Some(obj)) => s.push_item(Item::Obj(obj)),
                                Ok(None) => panic!("next obj should not be None"),
                                Err(err) => return Err(err.wrap("next obj")),
                            }
                        }
                        Some(Token::Lsquare) => {
                            let arr = self.next_arr();
                            match arr {
                                Ok(Some(arr)) => s.push_item(Item::Arr(arr)),
                                Ok(None) => panic!("next arr should not be None"),
                                Err(err) => return Err(err.wrap("next arr")),
                            }
                        }
                        Some(Token::Rparam) => {
                            // The end of this S. Drop the Rparam.
                            self.tokener.next();
                            break;
                        }
                        Some(token) => {
                            return Err(BuildError::new(&format!("unexcepted token {:?}", token)));
                        }
                        None => {
                            return Err(BuildError::new("unpaired s"));
                        },
                    }
                }
            },
            None => return Ok(None),
            Some(token) => return Err(
                BuildError::new(
                    &format!(
                        "first token want {:?}, got {:?}",
                        Token::Lbrace, token
                    )
                )
            ),
        }
        Ok(Some(s))
    }

    fn next_obj(&mut self) -> Result<Option<Object>, BuildError> {
        let mut obj = Object::new();
        let first_token = self.tokener.next();
        match first_token {
            Some(Token::Lbrace) => {
                // Time to build the Object.

                loop {
                    let next_token = self.tokener.next();
                    let key = match next_token {
                        Some(Token::Str(string)) => {
                            string
                        },
                        Some(Token::Rbrace) => {
                            break;
                        }
                        Some(token) => {
                            return Err(
                                BuildError::new(
                                    &format!(
                                        "try build object, expected a Token::Str as key, but got {:?}",
                                        token,
                                    )
                                )
                            )
                        },
                        None => {
                            return Err(
                                BuildError::new(
                                    "try build object, expected a Token::Str as key, but got nothing",
                                )
                            )
                        },
                    };
                    let colon = self.tokener.next();
                    if colon != Some(Token::Colon) {
                        return Err(
                            BuildError::new(
                                &format!(
                                    "try build object, expected a Token::Colon as key, but got {:?}",
                                    colon,
                                )
                            )
                        )
                    }
                    let value = self.tokener.peek();
                    let value = match value {
                        Some(Token::Str(_) | Token::I64(_)) => {
                            let next_token = self.tokener.next();
                            match next_token {
                                Some(Token::Str(sym)) => {
                                    Item::Str(sym)
                                },
                                Some(Token::I64(i64)) => {
                                    Item::I64(i64)
                                }
                                _ => panic!("next token must be str or i64"),
                            }
                        },
                        _ => return Err(
                            BuildError::new(
                                &format!(
                                    "try build object, expected a Token::Str or Token::I64 as value, but got {:?}",
                                    colon,
                                )
                            )
                        ),
                    };
                    obj.push_kv(key, value);

                    // Comsume the comma.
                    let last_token = self.tokener.peek();
                    match last_token {
                        Some(Token::Comma) => { self.tokener.next(); },
                        Some(Token::Rbrace) => { /* do nothing */ },
                        _ => return Err(BuildError::new(
                            &format!(
                                "try build object, expected a Token::Comma or Token::Rbrace after a KV, but got {:?}",
                                last_token,
                            )
                        ))
                    };
                }
            },
            None => return Ok(None),
            Some(token) => return Err(
                BuildError::new(
                    &format!(
                        "first token want {:?}, got {:?}",
                        Token::Lbrace, token
                    )
                )
            ),
        }

        Ok(Some(obj))
    }

    fn next_arr(&mut self) -> Result<Option<Array>, BuildError> {
        let mut arr = Array::new();
        let first_token = self.tokener.next();
        match first_token {
            Some(Token::Lsquare) => {
                // Time to build the Array.

                loop {
                    let value = self.tokener.peek();
                    let value = match value {
                        Some(Token::Sym(_)) => {
                            let next_token = self.tokener.next();
                            match next_token {
                                Some(Token::Sym(sym)) => {
                                    Item::Sym(sym)
                                },
                                _ => panic!("next token must be sym"),
                            }
                        },
                        Some(Token::Rsquare) => {
                            self.tokener.next();
                            break;
                        }
                        _ => return Err(
                            BuildError::new(
                                &format!(
                                    "try build array, expected a Token::Str as value, but got {:?}",
                                    value,
                                )
                            )
                        ),
                    };
                    arr.push(value);

                    // Comsume the comma.
                    let last_token = self.tokener.peek();
                    match last_token {
                        Some(Token::Comma) => { self.tokener.next(); },
                        Some(Token::Rsquare) => { /* do nothing */ },
                        _ => return Err(BuildError::new(
                            &format!(
                                "try build array, expected a Token::Comma or Token::Rsquare, but got {:?}",
                                last_token,
                            )
                        ))
                    };
                }
            },
            None => return Ok(None),
            Some(token) => return Err(
                BuildError::new(
                    &format!(
                        "first token want {:?}, got {:?}",
                        Token::Lsquare, token
                    )
                )
            ),
        }

        Ok(Some(arr))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BuildError {
    msg: String,
    inner: Option<Box<BuildError>>,
}

impl BuildError {
    fn new(msg: &str) -> Self {
        Self { msg: msg.to_string(), inner: None }
    }

    fn wrap(self, prefix: &str) -> Self {
        Self { msg: prefix.to_string(), inner: Some(Box::new(self)) }
    }

    fn to_string(&self) -> String {
        match self.inner {
            None => self.msg.clone(),
            Some(ref inner) => format!("{}: {}", self.msg, inner.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::item::Object;

    use super::*;

    #[test]
    fn basic() {
        let mut builder = Builder::new("()");
        assert_eq!(builder.build(), Ok(Ast::from([
            S::from([]),
        ])));

        let mut builder = Builder::new("(+ 1 1)");
        assert_eq!(builder.build(), Ok(Ast::from([
            S::from([
                Item::Sym("+".to_string()),
                Item::I64(1),
                Item::I64(1),
            ])
        ])));

        let mut builder = Builder::new(r###"
            (let a { "foo": "bar", "bar": 1 })
            (print a)
        "###);
        assert_eq!(builder.build(), Ok(Ast::from([
            S::from([
                Item::Sym("let".to_string()),
                Item::Sym("a".to_string()),
                Item::Obj(Object::from([
                    ("foo".to_string(), Item::Str("bar".to_string())),
                    ("bar".to_string(), Item::I64(1)),
                ])),
            ]),
            S::from([
                Item::Sym("print".to_string()),
                Item::Sym("a".to_string()),
            ])
        ])));

        let mut builder = Builder::new(r###"
            (-> (Image::from_url "http://foobar.com/example.png")
                (.resize { "width": 100, "height": 100 }))
        "###);
        assert_eq!(builder.build(), Ok(Ast::from([
            S::from([
                Item::Sym("->".to_string()),
                Item::S(S::from([
                    Item::Sym("Image::from_url".to_string()),
                    Item::Str("http://foobar.com/example.png".to_string()),
                ])),
                Item::S(S::from([
                    Item::Sym(".resize".to_string()),
                    Item::Obj(Object::from([
                        ("width".to_string(), Item::I64(100)),
                        ("height".to_string(), Item::I64(100)),
                    ]))
                ])),
            ]),
        ])));

        let mut builder = Builder::new(r###"
            (fn fac [n]
              (if (== n 0)
                1
                (begin
                  (let nxt (fac (- n 1)))
                  (* n nxt))))
            (print (fac 10))
        "###);
        assert_eq!(builder.build(), Ok(Ast::from([
            S::from([
                Item::Sym("fn".to_string()),
                Item::Sym("fac".to_string()),
                Item::Arr(Array::from([
                    Item::Sym("n".to_string())
                ])),
                Item::S(S::from([
                    Item::Sym("if".to_string()),
                    Item::S(S::from([
                        Item::Sym("==".to_string()),
                        Item::Sym("n".to_string()),
                        Item::I64(0),
                    ])),
                    Item::I64(1),
                    Item::S(S::from([
                        Item::Sym("begin".to_string()),
                        Item::S(S::from([
                            Item::Sym("let".to_string()),
                            Item::Sym("nxt".to_string()),
                            Item::S(S::from([
                                Item::Sym("fac".to_string()),
                                Item::S(S::from([
                                    Item::Sym("-".to_string()),
                                    Item::Sym("n".to_string()),
                                    Item::I64(1),
                                ])),
                            ])),
                        ])),
                        Item::S(S::from([
                            Item::Sym("*".to_string()),
                            Item::Sym("n".to_string()),
                            Item::Sym("nxt".to_string()),
                        ])),
                    ])),
                ]))
            ]),
            S::from([
                Item::Sym("print".to_string()),
                Item::S(S::from([
                    Item::Sym("fac".to_string()),
                    Item::I64(10),
                ])),
            ])
        ])));
    }
}