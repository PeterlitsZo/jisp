use std::{cell::RefCell, collections::HashMap};

use crate::{asm::{Asm, AsmStat}, ast::{Ast, SExp, SExpKind}, error::Error};

use super::AsmFn;

pub struct AsmBuilder<'a> {
    ast: Ast<'a>,
    name_idx: RefCell<HashMap<&'a str, u32>>,
}

impl<'a> AsmBuilder<'a> {
    /// Create a new [AsmBuilder] from [Ast].
    pub fn new(ast: Ast<'a>) -> Self {
        Self { ast, name_idx: RefCell::new(HashMap::new()) }
    }

    /// Consume self and return a built [Asm].
    pub fn build(self) -> Result<Asm, Error<'static>> {
        let mut asm_fn = AsmFn::new();
        let mut is_first_s_exp = true;
        let s_exps = self.ast.s_exps();
        for s_exp in s_exps {
            if is_first_s_exp {
                is_first_s_exp = false;
            } else {
                asm_fn.push_stat(AsmStat::Pop);
            }
            self.build_s_exp(&mut asm_fn, s_exp)?;
        }
        asm_fn.push_stat(AsmStat::Ret);

        let mut asm = Asm::new();
        asm.push_fn(asm_fn);
        Ok(asm)
    }

    fn build_s_exp<'s>(&'s self, asm_fn: &mut AsmFn, s_exp: &'s SExp<'a>) -> Result<(), Error<'static>> {
        match s_exp.kind() {
            SExpKind::Int => self.build_int(asm_fn, s_exp)?,
            SExpKind::List => self.build_list(asm_fn, s_exp)?,
            SExpKind::Name => {
                if let Some(idx) = self.name_idx.borrow().get(s_exp.as_name().unwrap()) {
                    asm_fn.push_stat(AsmStat::Load { idx: *idx });
                    return Ok(())
                }
                match s_exp.as_name().unwrap() {
                    "true" => asm_fn.push_stat(AsmStat::PushBool { val: true }),
                    "false" => asm_fn.push_stat(AsmStat::PushBool { val: false }),
                    _ => return Err(Error::todo("Expected variable, 'true' or 'false'.")),
                }
            }
        }
        Ok(())
    }

    fn build_int(&self, asm_fn: &mut AsmFn, s_exp: &SExp<'a>) -> Result<(), Error<'static>> {
        let val = s_exp.as_int().unwrap();
        asm_fn.push_stat(AsmStat::PushInt { val });
        Ok(())
    }

    fn build_list<'s>(&'s self, asm_fn: &mut AsmFn, s_exp: &'s SExp<'a>) -> Result<(), Error<'static>> {
        let lst: &'s [SExp<'a>] = s_exp.as_list().unwrap();
        if lst.is_empty() {
            return Err(Error::todo("Unexpected empty list."))
        }
        let name = match lst[0].kind() {
            SExpKind::Name => lst[0].as_name().unwrap(),
            _ => return Err(Error::todo(
                format!(
                    "Unexpected kind for list's first token: {}",
                    lst[0].kind().display(),
                )
            ))
        };

        #[derive(Clone, Copy)]
        enum Op {
            Add, Sub, Mul, Div, Mod,
            Eq, Ne, Lt, Le, Gt, Ge,
            Let,
        }
        let op = match name {
            "+" => Op::Add,
            "-" => Op::Sub,
            "*" => Op::Mul,
            "/" => Op::Div,
            "%" => Op::Mod,
            "==" => Op::Eq,
            "!=" => Op::Ne,
            "<" => Op::Lt,
            "<=" => Op::Le,
            ">" => Op::Gt,
            ">=" => Op::Ge,
            "let" => Op::Let,
            _ => return Err(Error::todo(format!(
                "Unexpected name for list's first token: {:?}.",
                name
            )))
        };

        match (op, lst.len() - 1) {
            (Op::Add, 0) => asm_fn.push_stat(AsmStat::PushInt { val: 0 }),
            (Op::Sub, 0) => return Err(Error::todo(
                "Unexpected args number 0.",
            )),
            (Op::Mul, 0) => asm_fn.push_stat(AsmStat::PushInt { val: 1 }),
            (Op::Div, 0) => return Err(Error::todo(
                "Unexpected args number 0.",
            )),
            (Op::Sub, 1) => {
                asm_fn.push_stat(AsmStat::PushInt { val: 0 });
                self.build_s_exp(asm_fn, &lst[1])?;
                asm_fn.push_stat(AsmStat::Sub);
            }
            (Op::Div, 1) => {
                asm_fn.push_stat(AsmStat::PushInt { val: 1 });
                self.build_s_exp(asm_fn, &lst[1])?;
                asm_fn.push_stat(AsmStat::Div);
            },
            (Op::Mod | Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge, 2) => {
                self.build_s_exp(asm_fn, &lst[1])?;
                self.build_s_exp(asm_fn, &lst[2])?;
                let stat = match op {
                    Op::Mod => AsmStat::Mod,
                    Op::Eq => AsmStat::Eq,
                    Op::Ne => AsmStat::Ne,
                    Op::Lt => AsmStat::Lt,
                    Op::Le => AsmStat::Le,
                    Op::Gt => AsmStat::Gt,
                    Op::Ge => AsmStat::Ge,
                    _ => panic!("unexpected op"),
                };
                asm_fn.push_stat(stat);
            }
            (Op::Let, 2) => {
                if let Some(name) = lst[1].as_name() {
                    self.name_idx.borrow_mut().insert(name, asm_fn.locals());
                } else {
                    return Err(Error::todo(format!(
                        "The 'let' s-exp's 2nd argument should be a symbol, got unexpected {}.",
                        lst[1].kind().display()
                    )))
                }
                self.build_s_exp(asm_fn, &lst[2])?;
                asm_fn.push_stat(AsmStat::Store { idx: asm_fn.locals() });
                asm_fn.push_stat(AsmStat::PushNull);

                asm_fn.set_locals(asm_fn.locals() + 1);
            }
            (Op::Add | Op::Sub | Op::Mul | Op::Div, _) => {
                let mut is_first = true;
                let stat = match op {
                    Op::Add => AsmStat::Add,
                    Op::Sub => AsmStat::Sub,
                    Op::Mul => AsmStat::Mul,
                    Op::Div => AsmStat::Div,
                    _ => panic!("unexpected op"),
                };
                for s_exp in &lst[1..] {
                    self.build_s_exp(asm_fn, s_exp)?;
                    if is_first {
                        is_first = false;
                    } else {
                        asm_fn.push_stat(stat);
                    }
                }
            }
            (_, num) => {
                return Err(Error::todo(format!(
                    "Unexpected args number {} for {:?}", num, name,
                )))
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{asm::AsmFn, ast::AstBuilder, ts::TokenStream};

    use super::*;

    fn test_asm_builder(source: &str, wanted: Asm) {
        let token_stream = TokenStream::new(source);
        let ast_builder = AstBuilder::new(token_stream);
        let ast = ast_builder.build().unwrap();
        let asm_builder = AsmBuilder::new(ast);
        let asm = asm_builder.build().unwrap();
        assert_eq!(asm, wanted);
    }

    #[test]
    fn compare() {
        test_asm_builder("(== true true)", Asm::from([
            AsmFn::from(0, [
                AsmStat::PushBool { val: true },
                AsmStat::PushBool { val: true },
                AsmStat::Eq,
                AsmStat::Ret,
            ])
        ]));
    }

    #[test]
    fn variable() {
        test_asm_builder("(let a 1) (let b 2) (+ a b)", Asm::from([
            AsmFn::from(2, [
                AsmStat::PushInt { val: 1 },
                AsmStat::Store { idx: 0 },
                AsmStat::PushNull,
                AsmStat::Pop,
                AsmStat::PushInt { val: 2 },
                AsmStat::Store { idx: 1 },
                AsmStat::PushNull,
                AsmStat::Pop,
                AsmStat::Load { idx: 0 },
                AsmStat::Load { idx: 1 },
                AsmStat::Add,
                AsmStat::Ret,
            ])
        ]));
    }
}
