use std::collections::HashMap;

use crate::{asm::asm_statement::AsmLabel, ast::{Ast, SExp}, value::Value};

use super::{asm::AsmFn, Asm, AsmStatement};

pub struct AsmBuilder {
    ast: Ast,

    locals_index: HashMap<String, u32>,
    label_cnt: u32,
    consts_index: HashMap<Value, u32>,
    consts: Vec<Value>,

    fns: Vec<AsmFn>,
    fns_index: HashMap<String, u32>,
}

impl AsmBuilder {
    pub fn new(ast: Ast) -> Self {
        Self {
            ast,

            locals_index: HashMap::new(),
            label_cnt: 1,
            consts_index: HashMap::new(),
            consts: vec![],

            fns: vec![],
            fns_index: HashMap::new(),
        }
    }

    pub fn build(mut self) -> Asm {
        let mut asm = Asm::new();

        let mut main_fn = AsmFn::new(0, vec![]);
        let ast = self.ast.clone();
        for s_exp in ast.s_exps() {
            self.build_value(&mut main_fn, s_exp);
        }
        main_fn.push_statement(AsmStatement::Ret);

        asm.consts = self.consts;
        asm.push_fn(main_fn);
        for func in self.fns {
            asm.push_fn(func);
        }
        asm
    }

    fn build_list(&mut self, asm_fn: &mut AsmFn, lst: &Vec<SExp>) {
        enum Op {
            Add, Sub, Mul, Div,
            Eq, Ne, Lt, Le, Gt, Ge,
            Let,
            If,
            Fn, Call,
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
            SExp::Sym(sym) if sym == &"if".to_string() => Op::If,
            SExp::Sym(sym) if sym == &"fn".to_string() => Op::Fn,
            _ => Op::Call,
        };

        match op {
            Op::Add | Op::Sub | Op::Mul | Op::Div => {
                self.build_value(asm_fn, &lst[1]);
                for i in 2..lst.len() {
                    self.build_value(asm_fn, &lst[i]);

                    match op {
                        Op::Add => asm_fn.push_statement(AsmStatement::Add),
                        Op::Sub => asm_fn.push_statement(AsmStatement::Sub),
                        Op::Mul => asm_fn.push_statement(AsmStatement::Mul),
                        Op::Div => asm_fn.push_statement(AsmStatement::Div),
                        _ => panic!("unexpected op"),
                    }
                }
            },
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => {
                for i in 1..=2 {
                    self.build_value(asm_fn, &lst[i]);
                }

                match op {
                    Op::Eq => asm_fn.push_statement(AsmStatement::Eq),
                    Op::Ne => asm_fn.push_statement(AsmStatement::Ne),
                    Op::Lt => asm_fn.push_statement(AsmStatement::Lt),
                    Op::Le => asm_fn.push_statement(AsmStatement::Le),
                    Op::Gt => asm_fn.push_statement(AsmStatement::Gt),
                    Op::Ge => asm_fn.push_statement(AsmStatement::Ge),
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
                            let ret = asm_fn.locals;
                            self.locals_index.insert(name, ret);
                            asm_fn.locals += 1;
                            ret
                        }
                    }
                };
                self.build_value(asm_fn, &lst[2]);
                asm_fn.push_statement(AsmStatement::Store { index });
            },
            Op::If => {
                self.build_value(asm_fn, &lst[1]);

                let fpath_label = AsmLabel::new(format!(".L{}", self.label_cnt));
                self.label_cnt += 1;
                let end_label = AsmLabel::new(format!(".L{}", self.label_cnt));
                self.label_cnt += 1;
                asm_fn.push_statement(AsmStatement::JumpFalse { label: fpath_label.clone() });

                // True path.
                self.build_value(asm_fn, &lst[2]);
                asm_fn.push_statement(AsmStatement::Jump { label: end_label.clone() });

                // False path.
                asm_fn.push_statement(AsmStatement::Label { label: fpath_label });
                self.build_value(asm_fn, &lst[3]);

                asm_fn.push_statement(AsmStatement::Label { label: end_label });
            },
            Op::Fn => {
                let mut func = AsmFn::new(0, vec![]);
                for s_exp in &lst[3..] {
                    self.build_value(&mut func, s_exp);
                }
                func.push_statement(AsmStatement::Ret);
                self.fns.push(func);
                let name = match &lst[1] {
                    SExp::Sym(name) => name.clone(),
                    _ => panic!("runtime error"),
                };
                self.consts.push(Value::Fn(self.fns.len() as u32));
                self.fns_index.insert(name, self.consts.len() as u32 - 1);
            },
            Op::Call => {
                let name = match &lst[0] {
                    SExp::Sym(name) => name.clone(),
                    _ => panic!("runtime error"),
                };
                asm_fn.push_statement(AsmStatement::PushConst {
                    index: *self.fns_index.get(&name).unwrap()
                });
                asm_fn.push_statement(AsmStatement::Call { args: 0 });
            }
        }
    }

    fn build_value(&mut self, asm_fn: &mut AsmFn, val: &SExp) {
        match val {
            SExp::I64(first) => {
                asm_fn.push_statement(AsmStatement::PushI64 { val: *first });
            }
            SExp::List(lst) => {
                self.build_list(asm_fn, lst);
            }
            SExp::Sym(name) => {
                let index = *self.locals_index.get(name).unwrap();
                asm_fn.push_statement(AsmStatement::Load { index });
            }
            SExp::Str(val) => {
                let ac = Value::Str(val.clone());
                let idx = match self.consts_index.get(&ac) {
                    None => {
                        self.consts.push(ac.clone());
                        let idx = self.consts.len() as u32 - 1;
                        self.consts_index.insert(ac.clone(), idx);
                        idx
                    }
                    Some(idx) => *idx,
                };
                asm_fn.push_statement(AsmStatement::PushConst { index: idx });
            }
            // TODO (@PeterlitsZo) Better error message.
            _ => panic!("we hope the second item is INTERGER or LIST")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::asm_statement::AsmLabel, ast::AstBuilder, token_stream::TokenStream};

    use super::*;

    #[test]
    fn basic() {
        let token_stream = TokenStream::new(r###"
            1
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);

        let token_stream = TokenStream::new(r###"
            (+ 1 2 3 4 5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.push_fn(AsmFn::new(0, vec![
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
        assert_eq!(asm, wanted);

        let token_stream = TokenStream::new(r###"
            (== 1 2)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::Eq,
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
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

        let mut wanted = Asm::new();
        wanted.push_fn(AsmFn::new(2, vec![
            AsmStatement::PushI64 { val: 12 },
            AsmStatement::Store { index: 0 },
            AsmStatement::PushI64 { val: 13 },
            AsmStatement::Store { index: 1 },
            AsmStatement::Load { index: 0 },
            AsmStatement::Load { index: 1 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
    }

    #[test]
    fn if_stmt() {
        let token_stream = TokenStream::new(r###"
            (if (== 2 1) 1 (* 2 1))
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Mul,
            AsmStatement::Label { label: AsmLabel::new(".L2") },
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
    }

    #[test]
    fn string() {
        let token_stream = TokenStream::new(r###"
            (let h "hello") (let w "world") (if (== 1 1) h w)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.consts = vec![
            Value::Str("hello".to_string()),
            Value::Str("world".to_string()),
        ];
        wanted.push_fn(AsmFn::new(2, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Store { index: 0 },

            AsmStatement::PushConst { index: 1 },
            AsmStatement::Store { index: 1 },

            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::Load { index: 0 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::Load { index: 1 },
            AsmStatement::Label { label: AsmLabel::new(".L2") },

            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
    }

    #[test]
    fn functions() {
        let token_stream = TokenStream::new(r###"
            (fn ret5 [] 5)
            (ret5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.consts = vec![
            Value::Fn(1),
        ];
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Call { args: 0 },
            AsmStatement::Ret,
        ]));
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
    }
}