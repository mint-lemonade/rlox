use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, interpreter::Interpreter};

mod error_reporter;
mod scanner;
mod token;
mod token_type;
mod expr;
mod ast_printer;
mod parser;
mod interpreter;
pub struct Lox {
    repl_mode: bool,
    interpreter: Interpreter
}

impl Lox {
    pub fn new(repl_mode: bool) -> Self {
        Self {
            repl_mode,
            interpreter: Interpreter {}
        }
    }

    pub fn run(&self, source: &str) {
        let error_reporter = ErrorReporter::new(
            source, self.repl_mode
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { return ; }     

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        if error_reporter.had_error.get() { return ; }

        ast_printer::pretty_print(ast.as_ref().unwrap());
        self.interpreter.interpret(ast.as_ref().unwrap(), &error_reporter);
        
        // for tkn in scanner.tokens {
        //     // dbg!(&tkn);
        // }
    }
}