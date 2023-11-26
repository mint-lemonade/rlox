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
    pub interpreter: Box<Interpreter<'p, T>>,
    source: Vec<String>
}

impl<'p, T: Print> Lox<'p, T> {
    pub fn new(repl_mode: bool, printer: &'p T) -> Self {
        Self {
            repl_mode,
            interpreter: Box::new(Interpreter::new(printer)),
            source: vec![],
        }
    }

    pub fn declaration_refs() -> Vec<Stmt> {
        vec![]
    }
    // pub fn run<'b>(&'b mut self, source: &'b str) where 'b: 'a {
    // pub fn run(&mut self, source: &'a str) {
    pub fn run(&mut self, source: String) -> i32 {
        self.source.push(source);
        let error_reporter = ErrorReporter::new(
            self.source.last().unwrap(), self.repl_mode, self.interpreter.printer
        );
        // let error_reporter = ErrorReporter::new(
        //     source, self.repl_mode
        // );

        let mut scanner = Scanner::new(self.source.last().unwrap(),  &error_reporter);
        // let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        // drop(parser);
        if error_reporter.had_error.get() { return 70; } // 70: An internal software error has been detected

        // self.ast.append(&mut ast);
        let mut declaration_refs = Self::declaration_refs();
        
        // TODO: This process exit code should be moved to main.rs
        // if error_reporter.had_runtime_error.get() && !self.repl_mode {
        //     return exit_code;
        // }
        self.interpreter.interpret(
            &ast, &error_reporter, &mut declaration_refs
        )
    }

    // pub fn run<'b>(&'b mut self, source: &'b str) where 'b: 'a {
    // // pub fn run(&mut self, source: &'a str) {
    //     // pub fn run(&mut self, source: String) {
    //         // self.source.push(source);
    //         // let error_reporter = ErrorReporter::new(
    //         //     self.source.last().unwrap(), self.repl_mode
    //         // );
    //         let error_reporter = ErrorReporter::new(
    //             source, self.repl_mode
    //         );
    
    //         // let mut scanner = Scanner::new(self.source.last().unwrap(),  &error_reporter);
    //         let mut scanner = Scanner::new(source,  &error_reporter);
            
    //         scanner.scan_tokens();
    //         if error_reporter.had_error.get() { return ; }     
    
    //         let parser = Parser::new(&scanner.tokens, &error_reporter);
    //         let ast = parser.parse();
    //         // drop(parser);
    //         if error_reporter.had_error.get() { return ; }
    
    //         // self.ast.append(&mut ast);
    //         self.interpreter.interpret(&ast, &error_reporter);
    //         // TODO: This process exit code should be moved to main.rs
    //         if !self.repl_mode {
    //             process::exit(70) // 70: An internal software error has been detected
    //         }
    //     }
}

