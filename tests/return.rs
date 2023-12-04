use std::fs;

use rlox::lox::{printer::TestPrinter, Lox};

#[test]
fn return_after_else() {
    let file_path = "./tests/return/after_else.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["ok".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn return_after_if() {
    let file_path = "./tests/return/after_if.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["ok".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn return_after_while() {
    let file_path = "./tests/return/after_while.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["ok".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn return_at_top_level() {
    let file_path = "./tests/return/at_top_level.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["Error: Can't return from top-level code.\n   line 1 | return \"wat\";".to_string()];
    assert_eq!(*printer.result.borrow(), result);

}

#[test]
fn return_in_function() {
    let file_path = "./tests/return/in_function.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["ok".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn return_nil_if_no_value() {
    let file_path = "./tests/return/return_nil_if_no_value.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec!["Nil".to_string()];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}