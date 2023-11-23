mod lox;
// mod printer;

use std::{env, process, io::{self, Write}, fs};

use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    #[allow(clippy::comparison_chain)]
    if args.len() > 2 {
        println!("Usage: jlox-rs [script]");
        process::exit(64); // 64: The command was used incorrectly
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
}

fn run_file(file_path: &str) {
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let mut lox_runner = Lox::new(false, &lox::printer::CliPrinter);
    let exit_code = lox_runner.run(source_code);
    process::exit(exit_code)
}

fn run_prompt() {
    let _lox_runner = Lox::new(true, &lox::printer::CliPrinter);
    let stdin = io::stdin();
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        match stdin.read_line(&mut input) {
            Ok(_) => {
                // println!("{input}");
                input.remove(input.len() - 1);
                // lox_runner.run(input);
            },
            Err(e) => println!("error: {e}"),
        }
        // input.clear();
        // lox_runner.had_error = false;
    }
}