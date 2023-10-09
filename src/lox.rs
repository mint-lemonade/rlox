use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser};

mod error_reporter;
mod scanner;
mod token;
mod token_type;
mod expr;
mod ast_printer;
mod parser;
pub struct Lox {
    repl_mode: bool,
}

impl Lox {
    pub fn new(repl_mode: bool) -> Self {
        Self {
            repl_mode
        }
    }

    pub fn run(&self, source: &str) {
        let error_reporter = ErrorReporter::new(
            source, self.repl_mode
        );

        let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();

        ast_printer::pretty_print(ast.as_ref().unwrap());
        
        // for tkn in scanner.tokens {
        //     // dbg!(&tkn);
        // }
    }
}