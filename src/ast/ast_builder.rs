use std::iter::Peekable;

use crate::token_stream::{Token, TokenStream};

use super::{Ast, SExp};

/// The builder of [Ast].
pub struct AstBuilder<'a> {
    token_stream: Peekable<TokenStream<'a>>,
}

impl<'a> AstBuilder<'a> {
    /// Build a [AstBuilder] from [TokenStream].
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream: token_stream.peekable() }
    }

    /// Build a [Ast].
    pub fn build(mut self) -> Ast {
        let mut ast = Ast::new();
        loop {
            let token = self.token_stream.peek();
            let token = match token {
                Some(t) => t.clone(),
                None => break,
            };
            let s_exp = match token {
                Token::I64(val) => {
                    self.skip(Token::I64(val));
                    SExp::I64(val)
                }
                Token::Lparam => {
                    self.next_list()
                }
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("unsupported token"),
            };
            ast.push_s_exp(s_exp);
        }
        ast
    }

    fn skip(&mut self, token: Token) {
        let next_token = self.token_stream.next();
        if next_token.unwrap() == token {
            return;
        }
        // TODO (@PeterlitsZo) Better error message.
        panic!("try skip but unexpected token")
    }

    fn next_list(&mut self) -> SExp {
        let mut result = vec![];
        self.skip(Token::Lparam);
        loop {
            let peek_token = self.token_stream.peek();
            let this_token = match peek_token {
                Some(Token::Rparam) => break,
                // TODO (@PeterlitsZo) Better error message.
                None => panic!("unexpected end"),
                Some(_) => self.token_stream.next().unwrap(),
            };
            let s_exp = match this_token {
                Token::I64(val) => SExp::I64(val),
                Token::Sym(sym) => SExp::Sym(sym),
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("unexpected token")
            };
            result.push(s_exp);
        }
        self.skip(Token::Rparam);
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
}