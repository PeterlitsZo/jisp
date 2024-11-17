mod error;
mod token_stream;
mod token;
mod value;
mod version;

use std::{fs, io, process::exit};

use clap::{arg, error::{ErrorKind, Result}, Command};
use token::TokenKind;
use token_stream::TokenStream;
use value::Value;

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
    let mut token_stream = TokenStream::new(code);

    let token = token_stream.next();
    let token = token.as_ref();
    if token.is_some_and(|t| t.kind() == TokenKind::Lparam) {
        let token = token_stream.next();
        let token = token.as_ref();
        let is_add = token.is_some_and(|t| {
            t.kind() == TokenKind::Name && t.val().as_name().unwrap() == "+"
        });
        let is_sub = token.is_some_and(|t| {
            t.kind() == TokenKind::Name && t.val().as_name().unwrap() == "-"
        });
        if !is_add && !is_sub {
            return Err(error::Error{});
        }
        let mut result = 0i64;
        let mut is_first = true;
        loop {
            let token = token_stream.next();
            let token = token.as_ref();
            if token.is_some_and(|t| t.kind() == TokenKind::Rparam) {
                break;
            }
            if !token.is_some_and(|t| t.kind() == TokenKind::Int) {
                return Err(error::Error{});
            }
            let num = token.unwrap().val().as_int_value().unwrap();
            let num = num.as_int().unwrap();
            if is_first {
                is_first = false;
                result = num;
            } else {
                result += if is_add { num } else { -num };
            }
        }
        Ok(Value::Int(result))
    } else if token.is_some_and(|t| t.kind() == TokenKind::Int) {
        let val = token.unwrap().val().as_int_value();
        match val {
            Some(val) => Ok(val),
            None => Err(error::Error{})
        }
    } else {
        Err(error::Error{})
    }
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