use std::{iter::Peekable, process::exit};

use crate::token_stream::{Token, TokenStream, TokenVal};

use super::{Ast, Error, ErrorMsg, SExp};

/// The builder of [Ast].
pub struct AstBuilder<'a> {
    source_plain: String,
    token_stream: Peekable<TokenStream<'a>>,
}

impl<'a> AstBuilder<'a> {
    /// Build a [AstBuilder] from [TokenStream].
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self {
            source_plain: token_stream.source_plain().to_string(),
            token_stream: token_stream.peekable()
        }
    }

    /// Build a [Ast].
    pub fn build(mut self) -> Ast {
        let mut ast = Ast::new();
        loop {
            let token = self.token_stream.peek();
            let token = match token {
                Some(t) if t.val() == &TokenVal::EOF => break,
                Some(t) => t.clone(),
                None => panic!("should not peek None"),
            };
            let s_exp = match token.val() {
                TokenVal::I64(val) => {
                    self.skip(TokenVal::I64(*val));
                    SExp::I64(*val)
                }
                TokenVal::Str(val) => {
                    self.skip(TokenVal::Str(val.clone()));
                    SExp::Str(val.clone())
                }
                TokenVal::Lparam => {
                    self.next_list()
                }
                _ => {
                    let err = Error::new(
                        &self.source_plain, token.pos(),
                        ErrorMsg::Unexpected { want: "I64 or LPARAM" }
                    );
                    err.print();
                    exit(1);
                },
            };
            ast.push_s_exp(s_exp);
        }
        ast
    }

    fn skip(&mut self, val: TokenVal) {
        let next_token = self.token_stream.next().unwrap();
        if next_token.val() == &val {
            return;
        }
        let err = Error::new(
            &self.source_plain, next_token.pos(),
            ErrorMsg::Unexpected { want: val.name() }
        );
        err.print();
        exit(1);
    }

    fn next_list(&mut self) -> SExp {
        let mut result = vec![];
        self.skip(TokenVal::Lparam);
        loop {
            let peek_token = self.token_stream.peek();
            let peek_token = match peek_token {
                Some(tok) if tok.val() == &TokenVal::Rparam => break,
                Some(tok) if tok.val() == &TokenVal::EOF => {
                    let err = Error::new(
                        &self.source_plain, tok.pos(),
                        ErrorMsg::Unexpected { want: "RPARAM, I64 or LPARAM" }
                    );
                    err.print();
                    exit(1);
                },
                Some(tok) => tok.clone(),
                None => panic!("should not peek None")
            };
            let s_exp = match peek_token.val() {
                TokenVal::Lparam => self.next_list(),
                TokenVal::I64(val) => {
                    self.skip(TokenVal::I64(*val));
                    SExp::I64(*val)
                }
                TokenVal::Str(val) => {
                    self.skip(TokenVal::Str(val.clone()));
                    SExp::Str(val.clone())
                }
                TokenVal::Sym(sym) => {
                    self.skip(TokenVal::Sym(sym.clone()));
                    SExp::Sym(sym.clone())
                }
                TokenVal::Lsquare => {
                    self.skip(TokenVal::Lsquare);
                    self.skip(TokenVal::Rsquare);
                    SExp::Array
                }
                _ => {
                    let err = Error::new(
                        &self.source_plain, peek_token.pos(),
                        ErrorMsg::Unexpected { want: "LPARAM, I64 or SYM" }
                    );
                    err.print();
                    exit(1);
                }
            };
            result.push(s_exp);
        }
        self.skip(TokenVal::Rparam);
        SExp::List(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let token_stream = TokenStream::new(r###"
            1
        "###);
        let ast = AstBuilder::new(token_stream).build();
        assert_eq!(ast, Ast::from([
            SExp::I64(1),
        ]));

        let token_stream = TokenStream::new(r###"
            (+ 1 2)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        assert_eq!(ast, Ast::from([
            SExp::List(vec![
                SExp::Sym("+".to_string()),
                SExp::I64(1),
                SExp::I64(2),
            ]),
        ]));
    }

    #[test]
    fn if_stmt() {
        let token_stream = TokenStream::new(r###"
            (if (== 2 1) 1 (* 2 1))
        "###);
        let ast = AstBuilder::new(token_stream).build();
        assert_eq!(ast, Ast::from([
            SExp::List(vec![
                SExp::Sym("if".to_string()),
                SExp::List(vec![
                    SExp::Sym("==".to_string()),
                    SExp::I64(2),
                    SExp::I64(1),
                ]),
                SExp::I64(1),
                SExp::List(vec![
                    SExp::Sym("*".to_string()),
                    SExp::I64(2),
                    SExp::I64(1),
                ]),
            ]),
        ]));
    }

    #[test]
    fn string() {
        let token_stream = TokenStream::new(r###"
            (let h "hello") (let w "world") (if (== 1 1) h w)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        assert_eq!(ast, Ast::from([
            SExp::List(vec![
                SExp::Sym("let".to_string()),
                SExp::Sym("h".to_string()),
                SExp::Str("hello".to_string()),
            ]),
            SExp::List(vec![
                SExp::Sym("let".to_string()),
                SExp::Sym("w".to_string()),
                SExp::Str("world".to_string()),
            ]),
            SExp::List(vec![
                SExp::Sym("if".to_string()),
                SExp::List(vec![SExp::Sym("==".to_string()), SExp::I64(1), SExp::I64(1)]),
                SExp::Sym("h".to_string()),
                SExp::Sym("w".to_string()),
            ]),
        ]));
    }

    #[test]
    fn functions() {
        let token_stream = TokenStream::new(r###"
            (fn ret5 [] 5)
            (ret5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        assert_eq!(ast, Ast::from([
            SExp::List(vec![
                SExp::Sym("fn".to_string()),
                SExp::Sym("ret5".to_string()),
                SExp::Array,
                SExp::I64(5),
            ]),
            SExp::List(vec![
                SExp::Sym("ret5".to_string()),
            ]),
        ]));
    }
}