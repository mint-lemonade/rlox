use std::fs;

use rlox::lox::{printer::TestPrinter, Lox};

#[test]
fn body_must_be_block() {
    let file_path = "./tests/function/body_must_be_block.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["Error: Expected '{' before function body.\n   line 3 | fun f() 123;".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn empty_body() {
    let file_path = "./tests/function/empty_body.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["Nil".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn local_mutual_recursion() {
    let file_path = "./tests/function/local_mutual_recursion.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["Error: Undefined variable 'isOdd'.\n   line 12 |   print isEven(3);".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn local_recursion() {
    let file_path = "./tests/function/local_recursion.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["21".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn nested_call_with_arguments() {
    let file_path = "./tests/function/nested_call_with_arguments.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["hello world".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}