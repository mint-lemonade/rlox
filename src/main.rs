mod lox;

use std::{env, process, io::{self, Write}, fs};

use lox::Lox;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
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
    let lox_runner = Lox::new(false);
    lox_runner.run(&source_code);
}

fn run_prompt() {
    let mut input = String::new();
    let mut lox_runner = Lox::new(true);
    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        match stdin.read_line(&mut input) {
            Ok(_) => {
                // println!("{input}");
                input.remove(input.len() - 1);
                lox_runner = lox_runner.run(&input);
            },
            Err(e) => println!("error: {e}"),
        }
        input.clear();
        // lox_runner.had_error = false;
    }
}