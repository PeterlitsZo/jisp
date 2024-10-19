use std::{env, fs};

mod token_stream;
mod ast;
mod asm;
mod bytecode;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).unwrap();
    let token_stream = token_stream::TokenStream::new(&content);
    let ast = ast::AstBuilder::new(token_stream).build();
    let asm = asm::AsmBuilder::new(ast).build();
    let bytecode = bytecode::BytecodeBuilder::new(asm).build();
    let val = bytecode::Runner::new(bytecode).run();
    println!("{}", val);
}
