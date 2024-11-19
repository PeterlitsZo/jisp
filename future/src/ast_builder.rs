use crate::{ast::Ast, error::Error, s_exp::SExp, token::{Token, TokenKind}, token_stream::TokenStream};

pub struct AstBuilder<'a> {
    token_stream: TokenStream<'a>,
    peek_token: Option<Token<'a>>,
}

impl<'a> AstBuilder<'a> {
    /// Create a new [AstBuilder] from [TokenStream].
    pub fn new(token_stream: TokenStream<'a>) -> Self {
        let mut result = Self { token_stream, peek_token: None };
        result.peek();
        result
    }

    /// Consume self and return a built [Ast].
    pub fn build(mut self) -> Result<Ast<'a>, Error> {
        let mut ast = Ast::new();
        loop {
            if self.peek_token.is_none() {
                break;
            }
            let s_exp = self.next_s_exp()?;
            ast.push_s_exp(s_exp);
        }
        Ok(ast)
    }

    fn peek(&mut self) {
        if self.peek_token.is_none() {
            self.peek_token = self.token_stream.next();
        }
    }

    fn consume(&mut self) {
        if self.peek_token.is_none() {
            panic!("unexpected None");
        };
        self.peek_token = None;
        self.peek();
    }

    fn skip(&mut self, kind: TokenKind) -> Result<(), Error> {
        let token = match &self.peek_token {
            None => panic!("unexpected None"),
            Some(t) => t,
        };
        if token.kind() != kind {
            return Err(Error::new());
        }
        self.consume();
        Ok(())
    }

    fn next_s_exp(&mut self) -> Result<SExp<'a>, Error> {
        match &self.peek_token {
            None => panic!("unexpected None"),
            Some(t) if t.kind() == TokenKind::Lparam => {
                self.next_list()
            },
            Some(t) if t.kind() == TokenKind::Name => {
                self.next_name()
            }
            Some(t) if t.kind() == TokenKind::Int => {
                self.next_int()
            }
            _ => todo!(),
        }
    }

    fn next_list(&mut self) -> Result<SExp<'a>, Error> {
        let mut result = vec![];

        self.skip(TokenKind::Lparam)?;
        loop {
            match &self.peek_token {
                Some(t) if t.kind() == TokenKind::Rparam => {
                    break;
                }
                _ => ()
            }
            result.push(self.next_s_exp()?);
        }
        self.skip(TokenKind::Rparam)?;

        Ok(SExp::List(result))
    }

    fn next_name(&mut self) -> Result<SExp<'a>, Error> {
        let t = self.peek_token.as_ref().unwrap();
        let result = Ok(SExp::Name(t.val().as_name().unwrap()));
        self.skip(TokenKind::Name)?;
        result
    }

    fn next_int(&mut self) -> Result<SExp<'a>, Error> {
        let t = self.peek_token.as_ref().unwrap();
        let result = Ok(SExp::Int(t.val().as_int().unwrap()));
        self.skip(TokenKind::Int)?;
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