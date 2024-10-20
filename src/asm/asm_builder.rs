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
                    self.build_list(&mut asm, lst);
                    asm.push_statement(AsmStatement::Ret);
                }
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("unexpected s_exp"),
            }
        }
        asm
    }

    fn build_list(&self, asm: &mut Asm, lst: &Vec<SExp>) {
        enum Op { Add, Sub, Mul, Div }

        let op = match &lst[0] {
            SExp::Sym(sym) if sym == &"+".to_string() => Op::Add,
            SExp::Sym(sym) if sym == &"-".to_string() => Op::Sub,
            SExp::Sym(sym) if sym == &"*".to_string() => Op::Mul,
            SExp::Sym(sym) if sym == &"/".to_string() => Op::Div,
            // TODO (@PeterlitsZo) Better error message.
            _ => panic!("we hope the first item is ADD, SUB, MUL or DIV")
        };

        match &lst[1] {
            SExp::I64(first) => {
                asm.push_statement(AsmStatement::PushI64 { val: *first });
            }
            SExp::List(lst) => {
                self.build_list(asm, lst);
            }
            // TODO (@PeterlitsZo) Better error message.
            _ => panic!("we hope the second item is INTERGER or LIST")
        };

        for i in 2..lst.len() {
            match &lst[i] {
                SExp::I64(val) => {
                    asm.push_statement(AsmStatement::PushI64 { val: *val });
                },
                SExp::List(lst) => {
                    self.build_list(asm, lst);
                }
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("we hope the item in rest is INTERGER")
            };

            match op {
                Op::Add => asm.push_statement(AsmStatement::AddI64),
                Op::Sub => asm.push_statement(AsmStatement::SubI64),
                Op::Mul => asm.push_statement(AsmStatement::MulI64),
                Op::Div => asm.push_statement(AsmStatement::DivI64),
            }
        }
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