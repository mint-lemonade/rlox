use std::process;

use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, interpreter::Interpreter};

mod error_reporter;
mod scanner;
mod token;
mod token_type;
mod expr;
mod ast_printer;
mod parser;
mod interpreter;
mod stmt;
mod environment;
pub struct Lox {
    repl_mode: bool,
    interpreter: Interpreter
}

impl Lox {
    pub fn new(repl_mode: bool) -> Self {
        Self {
            repl_mode,
            interpreter: Interpreter::new()
        }
    }

    pub fn run(&mut self, source: &str) {
        let error_reporter = ErrorReporter::new(
            source, self.repl_mode
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { return ; }     

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        if error_reporter.had_error.get() { return ; }

        self.interpreter.interpret(&ast, &error_reporter);
        // TODO: This process exit code should be moved to main.rs
        if !self.repl_mode {
            process::exit(70) // 70: An internal software error has been detected
        }
    }
}