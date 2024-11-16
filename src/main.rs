use std::{env, fs};

mod token_stream;
mod ast;
mod value;
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
    match val {
        value::Value::I64(val) => println!("{}", val),
        value::Value::Bool(val) => println!("{}", val),
        value::Value::Str(val) => println!("{:?}", val),
        _ => panic!("unexpected val type"),
    }
}
