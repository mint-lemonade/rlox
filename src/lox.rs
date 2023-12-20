pub mod printer;
mod error_reporter;
mod scanner;
mod token;
mod token_type;
mod expr;
mod parser;
mod resolver;
mod interpreter;
mod stmt;
mod environment;
mod callable;
mod class;
mod instance;

use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, interpreter::Interpreter, printer::Print, resolver::Resolver};


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

    pub fn run(&mut self, source: &str) -> i32 {
        let error_reporter = ErrorReporter::new(
            source, self.repl_mode, self.interpreter.printer
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        // if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        let mut resolver  = Resolver::new(&error_reporter, &mut self.interpreter);
        resolver.resolve(&ast);
        if error_reporter.had_error.get() { return 70; } // 70: An internal software 
        
        self.interpreter.interpret(
            &ast, &error_reporter
        )
    }
}

