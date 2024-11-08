mod asm;
mod asm_statement;
mod asm_builder;

pub type Asm = asm::Asm;
pub type AsmStatement = asm_statement::AsmStatement;
pub type AsmLabel = asm_statement::AsmLabel;
pub type AsmBuilder = asm_builder::AsmBuilder;