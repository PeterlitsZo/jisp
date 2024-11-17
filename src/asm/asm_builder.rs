use std::collections::HashMap;

use crate::{asm::asm_statement::AsmLabel, ast::{Ast, SExp}, value::{Value, XFn}};

use super::{asm::AsmFn, Asm, AsmStatement};

pub struct AsmBuilder {
    ast: Ast,

    consts_index: HashMap<Value, u32>,
    consts: Vec<Value>,

    fns_index: HashMap<String, u32>,
    ifns: Vec<AsmFn>,
    xfns: Vec<XFn>,
}

impl AsmBuilder {
    pub fn new(ast: Ast) -> Self {
        Self {
            ast,

            consts_index: HashMap::new(),
            consts: vec![],

            fns_index: HashMap::new(),
            ifns: vec![],
            xfns: vec![],
        }
    }

    pub fn register_xfn<F>(&mut self, name: String, xfn: F) where F: Fn(Vec<Value>) -> Value + 'static {
        let xfn = XFn::new(name.clone(), xfn);
        let xfn_value = Value::XFn(self.xfns.len() as u32);

        self.xfns.push(xfn);
        self.consts.push(xfn_value.clone());
        self.consts_index.insert(xfn_value, self.consts.len() as u32 - 1);
        self.fns_index.insert(name, self.consts.len() as u32 - 1);
    }

    pub fn build(mut self) -> Asm {
        let mut asm = Asm::new();

        let ast = self.ast.clone();
        let main_fn = AsmFnBuilder::new(&mut self)
            .build(ast);

        asm.consts = self.consts;
        asm.xfns = self.xfns;
        asm.push_fn(main_fn);
        for func in self.ifns {
            asm.push_fn(func);
        }
        asm
    }
}

pub struct AsmFnBuilder<'a> {
    ab: &'a mut AsmBuilder,

    locals_index: HashMap<String, u32>,
    label_cnt: u32,

    func: AsmFn,
}

impl<'a> AsmFnBuilder<'a> {
    fn new(ab: &'a mut AsmBuilder) -> Self {
        Self {
            ab,

            locals_index: HashMap::new(),
            label_cnt: 1,

            func: AsmFn::new(0, vec![]),
        }
    }

    fn build(mut self, ast: Ast) -> AsmFn {
        for s_exp in ast.s_exps() {
            self.build_value(s_exp);
        }
        self.func.push_statement(AsmStatement::Ret);
        self.func
    }

    fn build_list(&mut self, lst: &Vec<SExp>) {
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
                self.build_value(&lst[1]);
                for i in 2..lst.len() {
                    self.build_value(&lst[i]);

                    match op {
                        Op::Add => self.func.push_statement(AsmStatement::Add),
                        Op::Sub => self.func.push_statement(AsmStatement::Sub),
                        Op::Mul => self.func.push_statement(AsmStatement::Mul),
                        Op::Div => self.func.push_statement(AsmStatement::Div),
                        _ => panic!("unexpected op"),
                    }
                }
            },
            Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge => {
                for i in 1..=2 {
                    self.build_value(&lst[i]);
                }

                match op {
                    Op::Eq => self.func.push_statement(AsmStatement::Eq),
                    Op::Ne => self.func.push_statement(AsmStatement::Ne),
                    Op::Lt => self.func.push_statement(AsmStatement::Lt),
                    Op::Le => self.func.push_statement(AsmStatement::Le),
                    Op::Gt => self.func.push_statement(AsmStatement::Gt),
                    Op::Ge => self.func.push_statement(AsmStatement::Ge),
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
                            let ret = self.func.locals;
                            self.locals_index.insert(name, ret);
                            self.func.locals += 1;
                            ret
                        }
                    }
                };
                self.build_value(&lst[2]);
                self.func.push_statement(AsmStatement::Store { index });
            },
            Op::If => {
                self.build_value(&lst[1]);

                let fpath_label = AsmLabel::new(format!(".L{}", self.label_cnt));
                self.label_cnt += 1;
                let end_label = AsmLabel::new(format!(".L{}", self.label_cnt));
                self.label_cnt += 1;
                self.func.push_statement(AsmStatement::JumpFalse { label: fpath_label.clone() });

                // True path.
                self.build_value(&lst[2]);
                self.func.push_statement(AsmStatement::Jump { label: end_label.clone() });

                // False path.
                self.func.push_statement(AsmStatement::Label { label: fpath_label });
                self.build_value(&lst[3]);

                self.func.push_statement(AsmStatement::Label { label: end_label });
            },
            Op::Fn => {
                let name = match &lst[1] {
                    SExp::Sym(name) => name.clone(),
                    _ => panic!("runtime error"),
                };
                self.ab.consts.push(Value::IFn(self.ab.ifns.len() as u32 + 1));
                self.ab.fns_index.insert(name, self.ab.consts.len() as u32 - 1);

                let mut asm_fn_builder = AsmFnBuilder::new(self.ab);
                match &lst[2] {
                    SExp::Array(arr) => {
                        let mut idx = 0;
                        for ele in arr {
                            let name = match ele {
                                SExp::Sym(name) => name.clone(),
                                _ => panic!("argument should be a SYM"),
                            };
                            asm_fn_builder.locals_index.insert(name, idx);
                            idx += 1;
                        }
                        asm_fn_builder.func.locals = idx;
                    },
                    _ => panic!("arguments should be an ARRAY"),
                }
                let mut sub_ast = Ast::new();
                for s_exp in &lst[3..] {
                    sub_ast.push_s_exp(s_exp.clone());
                }
                let func = asm_fn_builder.build(sub_ast);
                self.ab.ifns.push(func);
            },
            Op::Call => {
                let name = match &lst[0] {
                    SExp::Sym(name) => name.clone(),
                    _ => panic!("runtime error"),
                };
                self.func.push_statement(AsmStatement::PushConst {
                    index: *self.ab.fns_index.get(&name).unwrap()
                });
                for val in &lst[1..] {
                    self.build_value(val);
                }
                self.func.push_statement(AsmStatement::Call { args: lst.len() as u32 - 1 });
            }
        }
    }

    fn build_value(&mut self, val: &SExp) {
        match val {
            SExp::I64(first) => {
                self.func.push_statement(AsmStatement::PushI64 { val: *first });
            }
            SExp::List(lst) => {
                self.build_list(lst);
            }
            SExp::Sym(name) => {
                let index = *self.locals_index.get(name).unwrap();
                self.func.push_statement(AsmStatement::Load { index });
            }
            SExp::Str(val) => {
                let ac = Value::Str(val.clone());
                let idx = match self.ab.consts_index.get(&ac) {
                    None => {
                        self.ab.consts.push(ac.clone());
                        let idx = self.ab.consts.len() as u32 - 1;
                        self.ab.consts_index.insert(ac.clone(), idx);
                        idx
                    }
                    Some(idx) => *idx,
                };
                self.func.push_statement(AsmStatement::PushConst { index: idx });
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
            Value::IFn(1),
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

        let token_stream = TokenStream::new(r###"
            (fn add [x y] (+ x y))
            (add 3 5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.consts = vec![
            Value::IFn(1),
        ];
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 3 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 2 },
            AsmStatement::Ret,
        ]));
        wanted.push_fn(AsmFn::new(2, vec![
            AsmStatement::Load { index: 0 },
            AsmStatement::Load { index: 1 },
            AsmStatement::Add,
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);

        let token_stream = TokenStream::new(r###"
            (fn fac [x]
              (if (== x 0)
                1
                (* (fac (- x 1)) x)))
            (fac 5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let asm = AsmBuilder::new(ast).build();

        let mut wanted = Asm::new();
        wanted.consts = vec![
            Value::IFn(1),
        ];
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 1 },
            AsmStatement::Ret,
        ]));
        wanted.push_fn(AsmFn::new(1, vec![
            AsmStatement::Load { index: 0 },
            AsmStatement::PushI64 { val: 0 },
            AsmStatement::Eq,
            AsmStatement::JumpFalse { label: AsmLabel::new(".L1") },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Jump { label: AsmLabel::new(".L2") },
            AsmStatement::Label { label: AsmLabel::new(".L1") },
            AsmStatement::PushConst { index: 0 },
            AsmStatement::Load { index: 0 },
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::Sub,
            AsmStatement::Call { args: 1 },
            AsmStatement::Load { index: 0 },
            AsmStatement::Mul,
            AsmStatement::Label { label: AsmLabel::new(".L2") },
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);

        let token_stream = TokenStream::new(r###"
            (x_add_3 5)
        "###);
        let ast = AstBuilder::new(token_stream).build();
        let mut asm_builder = AsmBuilder::new(ast);
        let x_add_3 = |args: Vec<Value>| {
            assert!(args.len() == 1);
            match args[0] {
                Value::I64(val) => Value::I64(val + 3),
                _ => panic!("unexpected value"),
            }
        };
        asm_builder.register_xfn("x_add_3".to_string(), x_add_3);
        let asm = asm_builder.build();

        let mut wanted = Asm::new();
        wanted.xfns = vec![
            XFn::new("x_add_3".to_string(), x_add_3),
        ];
        wanted.consts = vec![
            Value::XFn(0),
        ];
        wanted.push_fn(AsmFn::new(0, vec![
            AsmStatement::PushConst { index: 0 },
            AsmStatement::PushI64 { val: 5 },
            AsmStatement::Call { args: 1 },
            AsmStatement::Ret,
        ]));
        assert_eq!(asm, wanted);
    }
}