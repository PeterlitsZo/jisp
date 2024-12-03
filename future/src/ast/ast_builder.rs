use crate::{error::Error, ts::{Token, TokenKind, TokenStream}};
use super::{Ast, SExp};

pub struct AstBuilder<'a> {
    token_stream: TokenStream<'a>,
    peek_token: Token<'a>,
}

impl<'a> AstBuilder<'a> {
    /// Create a new [AstBuilder] from [TokenStream].
    pub fn new(mut token_stream: TokenStream<'a>) -> Self {
        let peek_token = token_stream.next();
        Self { token_stream, peek_token }
    }

    /// Consume self and return a built [Ast].
    pub fn build(mut self) -> Result<Ast<'a>, Error<'a>> {
        let mut ast = Ast::new();
        loop {
            if self.peek_token.is_eof() {
                break;
            }
            let s_exp = self.next_s_exp()?;
            ast.push_s_exp(s_exp);
        }
        Ok(ast)
    }

    fn consume(&mut self) {
        self.peek_token = self.token_stream.next();
    }

    fn skip(&mut self, kind: TokenKind) -> Result<(), Error<'static>> {
        if self.peek_token.kind() != kind {
            return Err(Error::todo(
                format!("Try skip {}, got {}.", kind.display(), self.peek_token.kind().display())
            ));
        }
        self.consume();
        Ok(())
    }

    fn next_s_exp(&mut self) -> Result<SExp<'a>, Error<'a>> {
        match self.peek_token.kind() {
            TokenKind::Lparam => {
                self.next_list()
            },
            TokenKind::Name => {
                self.next_name()
            }
            TokenKind::Int => {
                self.next_int()
            }
            TokenKind::Float => {
                self.next_float()
            }
            _ => Err(Error::todo("Unexpected token.")),
        }
    }

    fn next_list(&mut self) -> Result<SExp<'a>, Error<'a>> {
        let mut result = vec![];

        self.skip(TokenKind::Lparam)?;
        loop {
            match &self.peek_token.kind() {
                TokenKind::Rparam => {
                    break;
                }
                TokenKind::Eof => return Err(Error::syntax(
                    self.token_stream.origin(), self.peek_token.pos(), "Invalid syntax, maybe forgot ')'."
                )),
                _ => ()
            }
            result.push(self.next_s_exp()?);
        }
        self.skip(TokenKind::Rparam)?;

        Ok(SExp::List(result))
    }

    fn next_name(&mut self) -> Result<SExp<'a>, Error<'static>> {
        let t = &self.peek_token;
        let result = Ok(SExp::Name(t.val().as_name().unwrap()));
        self.skip(TokenKind::Name)?;
        result
    }

    fn next_int(&mut self) -> Result<SExp<'a>, Error<'static>> {
        let t = &self.peek_token;
        let result = Ok(SExp::Int(t.val().as_int().unwrap()));
        self.skip(TokenKind::Int)?;
        result
    }

    fn next_float(&mut self) -> Result<SExp<'a>, Error<'static>> {
        let t = &self.peek_token;
        let result = Ok(SExp::Float(t.val().as_float().unwrap()));
        self.skip(TokenKind::Float)?;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ast_builder(source: &str, wanted: Ast) {
        let token_stream = TokenStream::new(source);
        let ast_builder = AstBuilder::new(token_stream);
        let ast = ast_builder.build().unwrap();
        assert_eq!(ast, wanted);
    }

    #[test]
    fn calc() {
        test_ast_builder("1", Ast::from([
            SExp::Int(1),
        ]));

        test_ast_builder("(+ 1 1)", Ast::from([
            SExp::List(vec![
                SExp::Name("+"),
                SExp::Int(1),
                SExp::Int(1),
            ])
        ]));
    }
}