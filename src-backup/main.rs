mod ast;
mod bytecode;

fn main() {
    let mut builder = ast::Builder::new(r###"
       (+ 1 2)
    "###);
    let ast = builder.build().unwrap();
    let asm = bytecode::AsmBuilder::new(ast).build();
    let bytecode = bytecode::BytecodeBuilder::new(asm).build();
    let val = bytecode::Runner::new().run(bytecode);
    println!("{:?}", val);
}
