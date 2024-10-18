use crate::ast::{Ast, Item};

#[derive(Debug, PartialEq, Eq)]
struct Plain {
    statements: Vec<Statement>,
}

impl Plain {
    pub fn new() -> Self {
        Self { statements: vec![] }
    }

    pub fn from<T>(statements: T) -> Self where T: Into<Vec<Statement>> {
        Self { statements: statements.into() }
    }

    pub fn push_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Statement {
    Add { dst: StatementAtom, src1: StatementAtom, src2: StatementAtom },
    Ret { val: StatementAtom },
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum StatementAtom {
    I64(i64),
    Var(String),
    TempVar(u64),
}

struct TempVarGener {
    i: u64,
}

impl TempVarGener {
    fn new() -> Self {
        Self { i: 0 }
    }

    fn next(&mut self) -> StatementAtom {
        self.i += 1;
        StatementAtom::TempVar(self.i)
    }
}

struct PlainBuilder {
    ast: Ast,
    temp_var_gener: TempVarGener,
}

impl PlainBuilder {
    pub fn new(ast: Ast) -> Self {
        Self { ast, temp_var_gener: TempVarGener::new() }
    }

    pub fn build(&mut self) -> Result<Plain, ()> {
        let mut result = Plain::new();

        let mut ret = None;
        for s in self.ast.ss() {
            let add = Item::Sym("+".to_string());
            match s.car() {
                Some(add) => {
                    match s.cdr() {
                        Some(items) if items.len() >= 2 => {
                            let mut last_temp_var = self.temp_var_gener.next();
                            result.push_statement(Statement::Add {
                                dst: last_temp_var.clone(),
                                src1: self.ast_item_to_statement_atom(items[0].clone()),
                                src2: self.ast_item_to_statement_atom(items[1].clone()),
                            });
                            for i in 2..items.len() {
                                let this_temp_var = self.temp_var_gener.next();
                                result.push_statement(Statement::Add {
                                    dst: this_temp_var.clone(),
                                    src1: last_temp_var.clone(),
                                    src2: self.ast_item_to_statement_atom(items[1].clone()),
                                });
                                last_temp_var = this_temp_var.clone();
                            }
                            ret = Some(last_temp_var.clone())
                        }
                        _ => return Err(()),
                    }
                },
                _ => {
                    return Err(())
                }
            }
        }

        result.push_statement(Statement::Ret { val: ret.unwrap() });

        Ok(result)
    }

    fn ast_item_to_statement_atom(&self, item: Item) -> StatementAtom {
        match item {
            Item::I64(val) => StatementAtom::I64(val),
            _ => panic!("panic")
        }
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
        
        let mut plain_builder = PlainBuilder::new(ast);
        let plain = plain_builder.build().unwrap();

        assert_eq!(plain, Plain::from([
            Statement::Add {
                dst: StatementAtom::TempVar(1),
                src1: StatementAtom::I64(1),
                src2: StatementAtom::I64(2)
            },
            Statement::Ret { val: StatementAtom::TempVar(1) }
        ]));
    }
}