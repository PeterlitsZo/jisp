mod token_stream;
mod ast;
mod asm;
mod bytecode;

fn main() {
    let token_stream = token_stream::TokenStream::new(r###"
        (+ 1 1)
    "###);
    let ast = ast::AstBuilder::new(token_stream).build();
    let asm = asm::AsmBuilder::new(ast).build();
    let bytecode = bytecode::BytecodeBuilder::new(asm).build();
    let val = bytecode::Runner::new(bytecode).run();
    println!("{}", val);
}
