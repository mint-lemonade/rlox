use std::{cell::Cell, rc::Rc};

use super::{token::Token, printer::Print};

pub struct ErrorReporter<'a, T: Print> {
    repl_mode: bool,
    pub had_error: Cell<bool>,
    pub had_runtime_error: Cell<bool>,
    pub source_code: &'a str,
    pub printer: &'a T
}

impl<'a, T: Print> ErrorReporter<'a, T> {
    pub fn new(source_code: &'a str, repl_mode: bool, printer: &'a T) -> Self {
        Self {
            had_error: Cell::new(false),
            had_runtime_error: Cell::new(false),
            repl_mode,
            source_code,
            printer
        }
    }
    
    pub fn error(
        &self,
        line: usize, offset: usize,
        lexeme_length: usize, message: &str
    ) {
        self.printer.print(&self.format(
            line, offset, lexeme_length, message
        ));
        self.had_error.set(true);
    }

    pub fn error_token(&self, token: Rc<Token>, message: &str) {
        self.printer.print(&self.format(
            token.line, 0, 0, message
        ));
        self.had_error.set(true);
    }

    pub fn runtime_error(&self, token: Rc<Token>, message: String) {
        self.printer.print(&self.format(
            token.line, 0, 0, message.as_str()
        ));
        self.had_runtime_error.set(true);
    }

    fn format(&self,
        line: usize, _offset: usize,
        _lexeme_length: usize, message: &str
    ) -> String {
        let mut error_mssg = String::from("Error: ");
        error_mssg.push_str(message);
        error_mssg.push('.');
        if !self.repl_mode {
            error_mssg.push('\n');
            let error_line = self.source_code.split('\n').take(line).last().unwrap();
            error_mssg.push_str(format!("   line {line} | {error_line}").as_str());
        };
        error_mssg
    }
}

#[cfg(test)] 
mod tests {
    use crate::lox::printer::TestPrinter;

    use super::ErrorReporter;

    #[test]
    fn error_message_format_repl() {
        let source_code = "hello\nworld!";
        let printer = TestPrinter::default();
        let er = ErrorReporter::new(source_code, true, &printer);
        let msg = er.format(2, 3, 4, "Madeup Error");
        assert_eq!(msg, "Error: Madeup Error.")
    }

    #[test]
    fn error_message_format_script() {
        let source_code = "hello\nworld!";
        let printer = TestPrinter::default();
        let er = ErrorReporter::new(source_code, false, &printer);
        let msg = er.format(2, 3, 4, "Madeup Error");
        assert_eq!(msg, "Error: Madeup Error.\n   line 2 | world!");
    }
}