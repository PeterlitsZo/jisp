mod asm_builder;
mod asm_stat;
mod asm;
mod ast_builder;
mod ast;
mod bytecode_builder;
mod bytecode;
mod error;
mod runner;
mod s_exp;
mod token_stream;
mod token;
mod value;
mod version;

use std::{fs, io, process::exit};

use asm_builder::AsmBuilder;
use ast_builder::AstBuilder;
use bytecode_builder::BytecodeBuilder;
use clap::{arg, error::{ErrorKind, Result}, Command};
use runner::Runner;
use token_stream::TokenStream;

fn main() {
    let mut command = Command::new("jisp")
        .version(version::VERSION)
        .about("A JSON processing language.")
        .subcommand_required(true)
        .subcommand(
            Command::new("run")
                .about("Run jisp code.")
                .arg(arg!(-c --code <CODE> "Run jisp code from command line.").required(false))
                .arg(arg!([FILE] "Run jisp code from the given jisp file."))
        );
    let matches = command.clone().get_matches();

    match matches.subcommand() {
        Some(("run", sub_matches)) => {
            let cmd = sub_matches.get_one::<String>("code");
            if let Some(code) = cmd {
                let result = eval(code);
                handle_result(result);
                return;
            }

            let filename = sub_matches.get_one::<String>("FILE");
            if let Some(filename) = filename {
                let filecontent = match fs::read_to_string(filename) {
                    Ok(filecontent) => filecontent,
                    Err(err) => handle_error(&format!("open {:?}: ", filename), err),
                };
                let result = eval(&filecontent);
                handle_result(result);
                return;
            }

            let err = command.error(
                ErrorKind::MissingRequiredArgument, 
                "subcommand 'run': 'FILE' or code flag '--code' is required.");
            err.exit();
        },
        sc => {
            panic!("unexpected subcommand {:?}", sc)
        }
    }
}

fn handle_error(context: &str, err: io::Error) -> ! {
    println!("jisp: {}{}", context, err);
    exit(1);
}

fn eval(code: &str) -> Result<value::Value, error::Error> {
    let token_stream = TokenStream::new(code);
    let ast_builder = AstBuilder::new(token_stream);
    let ast = ast_builder.build()?;
    let asm_builder = AsmBuilder::new(ast);
    let asm = asm_builder.build()?;
    let bytecode_builder = BytecodeBuilder::new(asm);
    let bytecode = bytecode_builder.build();
    let runner = Runner::new(bytecode);
    let result = runner.run();
    Ok(result)
}

fn handle_result(r: Result<value::Value, error::Error>) {
    match r {
        Err(_) => {
            eprintln!("jisp: something error happend");
            exit(1);
        }
        Ok(v) => print!("{}", v.display()),
    }
}