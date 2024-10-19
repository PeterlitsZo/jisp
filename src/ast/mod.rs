mod ast;
mod ast_builder;
mod s_exp;
mod error;

pub type Ast = ast::Ast;
pub type SExp = s_exp::SExp;
pub type AstBuilder<'a> = ast_builder::AstBuilder<'a>;
pub type Error<'a, 'b> = error::Error<'a, 'b>;
pub type ErrorMsg<'a> = error::ErrorMsg<'a>;