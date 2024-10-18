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
            match s_exp {
                SExp::I64(val) => {
                    asm.push_statement(AsmStatement::PushI64 { val: *val });
                    asm.push_statement(AsmStatement::Ret);
                }
                SExp::List(lst) => {
                    if lst.len() != 3 {
                        // TODO (@PeterlitsZo) Better error message.
                        panic!("the lst's length should be 3")
                    }
                    match &lst[0] {
                        SExp::Sym(sym) if sym == &"+".to_string() => {}
                        // TODO (@PeterlitsZo) Better error message.
                        _ => panic!("we hope the first item is PLUS")
                    }
                    let first = match &lst[1] {
                        SExp::I64(first) => *first,
                        // TODO (@PeterlitsZo) Better error message.
                        _ => panic!("we hope the second item is INTERGER")
                    };
                    let second = match &lst[2] {
                        SExp::I64(second) => *second,
                        // TODO (@PeterlitsZo) Better error message.
                        _ => panic!("we hope the third item is INTERGER")
                    };
                    asm.push_statement(AsmStatement::PushI64 { val: first });
                    asm.push_statement(AsmStatement::PushI64 { val: second });
                    asm.push_statement(AsmStatement::AddI64);
                    asm.push_statement(AsmStatement::Ret);
                }
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("unexpected s_exp"),
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

        let token_stream = TokenStream::new(r###"
            (+ 1 2)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        assert_eq!(asm, Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::AddI64,
            AsmStatement::Ret,
        ]));
    }
}