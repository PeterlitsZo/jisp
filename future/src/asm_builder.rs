use crate::{asm::Asm, asm_stat::AsmStat, ast::Ast, error::Error, s_exp::{SExp, SExpKind}};

pub struct AsmBuilder<'a> {
    ast: Ast<'a>,
}

impl<'a> AsmBuilder<'a> {
    /// Create a new [AsmBuilder] from [Ast].
    pub fn new(ast: Ast<'a>) -> Self {
        Self { ast }
    }

    /// Consume self and return a built [Asm].
    pub fn build(self) -> Result<Asm, Error> {
        let mut asm = Asm::new();
        let mut is_first_s_exp = true;
        for s_exp in self.ast.s_exps() {
            Self::build_s_exp(&mut asm, s_exp)?;
            if is_first_s_exp {
                is_first_s_exp = false;
            } else {
                asm.push_stat(AsmStat::Pop);
            }
        }
        asm.push_stat(AsmStat::Ret);
        Ok(asm)
    }

    fn build_s_exp(asm: &mut Asm, s_exp: &SExp<'a>) -> Result<(), Error> {
        match s_exp.kind() {
            SExpKind::Int => Self::build_int(asm, s_exp)?,
            SExpKind::List => Self::build_list(asm, s_exp)?,
            SExpKind::Name => {
                match s_exp.as_name().unwrap() {
                    "true" => asm.push_stat(AsmStat::PushBool { val: true }),
                    "false" => asm.push_stat(AsmStat::PushBool { val: false }),
                    _ => return Err(Error{})
                }
            }
        }
        Ok(())
    }

    fn build_int(asm: &mut Asm, s_exp: &SExp<'a>) -> Result<(), Error> {
        let val = s_exp.as_int().unwrap();
        asm.push_stat(AsmStat::PushInt { val });
        Ok(())
    }

    fn build_list(asm: &mut Asm, s_exp: &SExp<'a>) -> Result<(), Error> {
        let lst = s_exp.as_list().unwrap();
        if lst.is_empty() {
            return Err(Error{})
        }
        let name = match lst[0].kind() {
            SExpKind::Name => lst[0].as_name().unwrap(),
            _ => return Err(Error{})
        };

        #[derive(Clone, Copy)]
        enum Op { Add, Sub, Mul, Div, Mod, Eq, Ne, Lt, Le, Gt, Ge }
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
            _ => return Err(Error{})
        };

        match (op, lst.len() - 1) {
            (Op::Add, 0) => asm.push_stat(AsmStat::PushInt { val: 0 }),
            (Op::Sub, 0) => return Err(Error {}),
            (Op::Mul, 0) => asm.push_stat(AsmStat::PushInt { val: 1 }),
            (Op::Div, 0) => return Err(Error {}),
            (Op::Sub, 1) => {
                asm.push_stat(AsmStat::PushInt { val: 0 });
                Self::build_s_exp(asm, &lst[1])?;
                asm.push_stat(AsmStat::Sub);
            }
            (Op::Div, 1) => {
                asm.push_stat(AsmStat::PushInt { val: 1 });
                Self::build_s_exp(asm, &lst[1])?;
                asm.push_stat(AsmStat::Div);
            },
            (Op::Mod | Op::Eq | Op::Ne | Op::Lt | Op::Le | Op::Gt | Op::Ge, 2) => {
                Self::build_s_exp(asm, &lst[1])?;
                Self::build_s_exp(asm, &lst[2])?;
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
                asm.push_stat(stat);
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
                    Self::build_s_exp(asm, s_exp)?;
                    if is_first {
                        is_first = false;
                    } else {
                        asm.push_stat(stat);
                    }
                }
            }
            (_, _) => {
                return Err(Error{})
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast_builder::AstBuilder, token_stream::TokenStream};

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
            AsmStat::PushBool { val: true },
            AsmStat::PushBool { val: true },
            AsmStat::Eq,
            AsmStat::Ret,
        ]));
    }
}
