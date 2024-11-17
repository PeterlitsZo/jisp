mod token_stream;
mod ast;
mod value;
mod asm;
mod bytecode;

use std::{env, fs};

use value::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let content = fs::read_to_string(&args[1]).unwrap();
    let token_stream = token_stream::TokenStream::new(&content);
    let ast = ast::AstBuilder::new(token_stream).build();
    let mut asm_builder = asm::AsmBuilder::new(ast);
    asm_builder.register_xfn("x_fac".to_string(), |args: Vec<Value>| {
        fn fac(n: i64) -> i64 {
            if n == 0 { 1 } else { fac(n - 1) * n }
        }
        assert!(args.len() == 1);
        match args[0] {
            Value::I64(val) => Value::I64(fac(val)),
            _ => panic!("unexpected value"),
        }
    });
    let asm = asm_builder.build();
    let bytecode = bytecode::BytecodeBuilder::new(asm).build();
    let val = bytecode::Runner::new(bytecode).run();
    match val {
        Value::I64(val) => println!("{}", val),
        Value::Bool(val) => println!("{}", val),
        Value::Str(val) => println!("{:?}", val),
        _ => panic!("unexpected val type"),
    }
}
