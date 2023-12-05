use std::fs;

use rlox::lox::{printer::TestPrinter, Lox};

#[test]
fn body_must_be_block() {
    let file_path = "./tests/closure/assign_to_closure.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "local", "after f", "after f", "after g"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn close_over_later_variable() {
    let file_path = "./tests/closure/close_over_later_variable.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "b", "a",
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn nested_closure() {
    let file_path = "./tests/closure/nested_closure.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "a", "b", "c"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn reuse_closure_slot() {
    let file_path = "./tests/closure/reuse_closure_slot.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "a"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn unused_later_closure() {
    let file_path = "./tests/closure/unused_later_closure.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "a"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn assign_to_shadowed_later() {
    let file_path = "./tests/closure/assign_to_shadowed_later.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "inner", "assigned"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn close_over_method_parameter() {
    let file_path = "./tests/closure/close_over_method_parameter.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "param" 
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn open_closure_in_function() {
    let file_path = "./tests/closure/open_closure_in_function.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "local"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn shadow_closure_with_local() {
    let file_path = "./tests/closure/shadow_closure_with_local.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "closure", "shadow", "closure"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn close_over_function_parameter() {
    let file_path = "./tests/closure/close_over_function_parameter.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "param"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn closed_closure_in_function() {
    let file_path = "./tests/closure/closed_closure_in_function.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "local"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn reference_closure_multiple_times() {
    let file_path = "./tests/closure/reference_closure_multiple_times.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "a", "a"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn unused_closure() {
    let file_path = "./tests/closure/unused_closure.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "ok"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}