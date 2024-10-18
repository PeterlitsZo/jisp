use crate::ast::{Ast, Item};

#[derive(Debug, PartialEq, Eq)]
pub struct Asm {
    pub statements: Vec<AsmStatement>,
}

impl Asm {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn from<T>(statements: T) -> Self where T: Into<Vec<AsmStatement>> {
        Self { statements: statements.into() }
    }

    pub fn push_statement(&mut self, statement: AsmStatement) {
        self.statements.push(statement);
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AsmStatement {
    Ret,
    PushI64 { val: i64 },
    AddI64,
    SubI64,
}

pub struct AsmBuilder {
    pub ast: Ast,
}

impl AsmBuilder {
    pub fn new(ast: Ast) -> Self {
        Self { ast }
    }

    pub fn build(&self) -> Asm {
        let mut asm = Asm::new();

        for s in self.ast.ss() {
            let add_fn = Item::Sym("+".to_string());
            match s.car() {
                Some(add_fn) => {
                    match s.cdr() {
                        Some(items) if items.len() == 2 => {
                            match items[0] {
                                Item::I64(val) => {
                                    asm.push_statement(AsmStatement::PushI64 { val: val });
                                },
                                _ => todo!(),
                            }
                            match items[1] {
                                Item::I64(val) => {
                                    asm.push_statement(AsmStatement::PushI64 { val: val });
                                },
                                _ => todo!(),
                            }
                            asm.push_statement(AsmStatement::AddI64);
                        }
                        _ => todo!(),
                    }
                },
                _ => todo!(),
            }
        }

        asm.push_statement(AsmStatement::Ret);
        asm
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::Builder;

    use super::*;

    #[test]
    fn basic() {
        let mut ast_builder = Builder::new(r###"
            (+ 1 2)
        "###);
        let ast = ast_builder.build().unwrap();
        
        let asm_builder = AsmBuilder::new(ast);
        let asm = asm_builder.build();

        assert_eq!(asm, Asm::from([
            AsmStatement::PushI64 { val: 1 },
            AsmStatement::PushI64 { val: 2 },
            AsmStatement::AddI64,
            AsmStatement::Ret,
        ]));
    }
}