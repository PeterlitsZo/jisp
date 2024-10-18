use crate::token_stream::{Token, TokenStream};

use super::{Ast, SExp};

/// The builder of [Ast].
pub struct AstBuilder<'a> {
    token_stream: TokenStream<'a>,
}

impl<'a> AstBuilder<'a> {
    /// Build a [AstBuilder] from [TokenStream].
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        Self { token_stream }
    }

    /// Build a [Ast].
    pub fn build(mut self) -> Ast {
        let mut ast = Ast::new();
        loop {
            let token = self.token_stream.next();
            let token = match token {
                Some(t) => t,
                None => break,
            };
            let s_exp = match token {
                Token::I64(val) => SExp::I64(val),
            };
            ast.push_s_exp(s_exp);
        }
        ast
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
    }
}