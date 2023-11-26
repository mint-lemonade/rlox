pub mod printer;
mod error_reporter;
mod scanner;
mod token;
mod token_type;
mod expr;
mod parser;
mod interpreter;
mod stmt;
mod environment;
mod callable;

use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, interpreter::Interpreter, stmt::Stmt, printer::Print};


pub struct Lox<'p, T: Print> {
    repl_mode: bool,
    pub interpreter: Interpreter<'p, T>
}

impl<'p, T: Print> Lox<'p, T> {
    pub fn new(repl_mode: bool, printer: &'p T) -> Self {
        Self {
            repl_mode,
            interpreter: Interpreter::new(printer)
        }
    }

    pub fn declaration_refs() -> Vec<Stmt> {
        vec![]
    }

    pub fn run(&mut self, source: &str) -> i32 {
        let error_reporter = ErrorReporter::new(
            source, self.repl_mode, self.interpreter.printer
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        // drop(parser);
        if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        // self.ast.append(&mut ast);
        let mut declaration_refs = Self::declaration_refs();
        
        self.interpreter.interpret(
            &ast, &error_reporter, &mut declaration_refs
        )
    }
}

