use std::collections::HashMap;

use crate::ast::{Ast, SExp};

use super::{Asm, AsmStatement};

pub struct AsmBuilder {
    ast: Ast,

    locals_index: HashMap<String, u32>,
}

impl AsmBuilder {
    pub fn new(ast: Ast) -> Self {
        Self {
            ast,
            locals_index: HashMap::new(),
        }
    }

    pub fn build(mut self) -> Asm {
        let mut asm = Asm::new(0);

        let ast = self.ast.clone();
        for s_exp in ast.s_exps() {
            match s_exp {
                SExp::I64(val) => {
                    asm.push_statement(AsmStatement::PushI64 { val: *val });
                }
                SExp::List(lst) => {
                    self.build_list(&mut asm, lst);
                }
                // TODO (@PeterlitsZo) Better error message.
                _ => panic!("unexpected s_exp"),
            }
        }
        asm.push_statement(AsmStatement::Ret);
        asm
    }

    fn build_list(&mut self, asm: &mut Asm, lst: &Vec<SExp>) {
        enum Op {
            Add, Sub, Mul, Div,
            Eq, Ne, Lt, Le, Gt, Ge,
            Let,
        }

        let op = match &lst[0] {
            SExp::Sym(sym) if sym == &"+".to_string() => Op::Add,
            SExp::Sym(sym) if sym == &"-".to_string() => Op::Sub,
            SExp::Sym(sym) if sym == &"*".to_string() => Op::Mul,
            SExp::Sym(sym) if sym == &"/".to_string() => Op::Div,
            SExp::Sym(sym) if sym == &"==".to_string() => Op::Eq,
            SExp::Sym(sym) if sym == &"!=".to_string() => Op::Ne,
            SExp::Sym(sym) if sym == &"<".to_string() => Op::Lt,
            SExp::Sym(sym) if sym == &"<=".to_string() => Op::Le,
            SExp::Sym(sym) if sym == &">".to_string() => Op::Gt,
            SExp::Sym(sym) if sym == &">=".to_string() => Op::Ge,
            SExp::Sym(sym) if sym == &"let".to_string() => Op::Let,
            // TODO (@PeterlitsZo) Better error message.
            _ => panic!("unexpected first item")
        };

        match op {
            Op::Add | Op::Sub | Op::Mul | Op::Div => {
                self.build_value(asm, &lst[1]);
                for i in 2..lst.len() {
                    self.build_value(asm, &lst[i]);

                    match op {
                        Op::Add => asm.push_statement(AsmStatement::Add),
                        Op::Sub => asm.push_statement(AsmStatement::Sub),
                        Op::Mul => asm.push_statement(AsmStatement::Mul),
                        Op::Div => asm.push_statement(AsmStatement::Div),
                        _ => panic!("unexpected op"),
                    }
                }
            },
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => {
                for i in 1..=2 {
                    self.build_value(asm, &lst[i]);
                }

                match op {
                    Op::Eq => asm.push_statement(AsmStatement::Eq),
                    Op::Ne => asm.push_statement(AsmStatement::Ne),
                    Op::Lt => asm.push_statement(AsmStatement::Lt),
                    Op::Le => asm.push_statement(AsmStatement::Le),
                    Op::Gt => asm.push_statement(AsmStatement::Gt),
                    Op::Ge => asm.push_statement(AsmStatement::Ge),
                    _ => panic!("unexpected op"),
                }
            },
            Op::Let => {
                let index = {
                    let name = match &lst[1] {
                        SExp::Sym(sym) => sym.clone(),
                        _ => panic!("unexpected token"),
                    };
                    match self.locals_index.get(&name) {
                        Some(idx) => *idx,
                        None => {
                            let ret = asm.locals;
                            self.locals_index.insert(name, ret);
                            asm.locals += 1;
                            ret
                        }
                    }
                };
                self.build_value(asm, &lst[2]);
                asm.push_statement(AsmStatement::Store { index });
            }
        }
    }

    fn build_value(&mut self, asm: &mut Asm, val: &SExp) {
        match val {
            SExp::I64(first) => {
                asm.push_statement(AsmStatement::PushI64 { val: *first });
            }
            SExp::List(lst) => {
                self.build_list(asm, lst);
            }
            SExp::Sym(name) => {
                let index = *self.locals_index.get(name).unwrap();
                asm.push_statement(AsmStatement::Load { index });
            }
            // TODO (@PeterlitsZo) Better error message.
            _ => panic!("we hope the second item is INTERGER or LIST")
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

        assert_eq!(asm, Asm::from(0, [
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Ret,
        ]));

        let token_stream = TokenStream::new(r###"
            (+ 1 2 3 4 5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        assert_eq!(asm, Asm::from(0, [
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Add,
            AsmStatement::PushI64 { val: 3 },
            AsmStatement::Add,
            AsmStatement::PushI64 { val: 4 },
            AsmStatement::Add,
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));

        let token_stream = TokenStream::new(r###"
            (== 1 2)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        assert_eq!(asm, Asm::from(0, [
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ]));
    }

    #[test]
    fn locals() {
        let token_stream = TokenStream::new(r###"
            (let a 12)
            (let b 13)
            (+ a b)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        assert_eq!(asm, Asm::from(2, [
            AsmStatement::PushI64 { val: 12 },
            AsmStatement::Store { index: 0 },
            AsmStatement::PushI64 { val: 13 },
            AsmStatement::Store { index: 1 },
            AsmStatement::Load { index: 0 },
            AsmStatement::Load { index: 1 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
    }
}