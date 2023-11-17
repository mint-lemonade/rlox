use std::process;

use self::{error_reporter::ErrorReporter, scanner::Scanner, parser::Parser, interpreter::Interpreter, stmt::Stmt};

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
mod callable;
pub struct Lox {
    repl_mode: bool,
    interpreter: Box<Interpreter>,
    source: Vec<String>
}

impl Lox {
    pub fn new(repl_mode: bool) -> Self {
        Self {
            repl_mode,
            interpreter: Box::new(Interpreter::new()),
            source: vec![],
        }
    }

    // pub fn run<'b>(&'b mut self, source: &'b str) where 'b: 'a {
    // pub fn run(&mut self, source: &'a str) {
    pub fn run(&mut self, source: String) {
        self.source.push(source);
        let error_reporter = ErrorReporter::new(
            self.source.last().unwrap(), self.repl_mode
        );
        // let error_reporter = ErrorReporter::new(
        //     source, self.repl_mode
        // );

        let mut scanner = Scanner::new(self.source.last().unwrap(),  &error_reporter);
        // let mut scanner = Scanner::new(source,  &error_reporter);
        
        scanner.scan_tokens();
        if error_reporter.had_error.get() { return ; }     

        let parser = Parser::new(&scanner.tokens, &error_reporter);
        let ast = parser.parse();
        // drop(parser);
        if error_reporter.had_error.get() { return ; }

        // self.ast.append(&mut ast);
        self.interpreter.interpret(&ast, &error_reporter);
        // TODO: This process exit code should be moved to main.rs
        if !self.repl_mode {
            process::exit(70) // 70: An internal software error has been detected
        }
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