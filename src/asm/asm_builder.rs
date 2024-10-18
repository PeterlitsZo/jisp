use crate::ast::{Ast, SExp};

use super::{Asm, AsmStatement};

pub struct AsmBuilder {
    ast: Ast,
}

impl AsmBuilder {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn build(self) -> Asm {
        let mut asm = Asm::new();

        for s_exp in self.ast.s_exps() {
            match *s_exp {
                SExp::I64(val) => {
                    asm.push_statement(AsmStatement::PushI64 { val });
                    asm.push_statement(AsmStatement::Ret);
                }
            }
        }
        asm
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast::AstBuilder, token_stream::TokenStream};

    use super::*;

    #[test]
    fn basic() {
        let token_stream = TokenStream::new(r###"
            1
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        assert_eq!(asm, Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Ret,
        ]));
    }
}