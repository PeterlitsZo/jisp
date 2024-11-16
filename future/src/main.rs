mod version;

use std::{fs, io, process::exit};

use clap::{arg, error::ErrorKind, Command};

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
                print!("{}", result);
                return;
            }

            let filename = sub_matches.get_one::<String>("FILE");
            if let Some(filename) = filename {
                let filecontent = match fs::read_to_string(filename) {
                    Ok(filecontent) => filecontent,
                    Err(err) => handle_error(&format!("open {:?}: ", filename), err),
                };
                let result = eval(&filecontent);
                print!("{}", result);
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

fn eval(code: &str) -> &str {
    code
}
