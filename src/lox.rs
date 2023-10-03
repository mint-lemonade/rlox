use self::{error_reporter::ErrorReporter, scanner::Scanner};

mod error_reporter;
mod scanner;
mod token;
mod token_type;
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
        
        for tkn in scanner.tokens {
            dbg!(&tkn);
        }
    }
}