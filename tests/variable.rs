use std::fs;

use rlox::lox::{printer::TestPrinter, Lox};

#[test]
fn collide_with_parameter() {
    let file_path = "./tests/variable/collide_with_parameter.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Already a variable with this name in this scope."));
}
#[test]
fn in_middle_of_block() {
    let file_path = "./tests/variable/in_middle_of_block.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "a", "a b", "a c", "a b d"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn redefine_global() {
    let file_path = "./tests/variable/redefine_global.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "2"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn shadow_local() {
    let file_path = "./tests/variable/shadow_local.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "shadow", "local"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn unreached_undefined() {
    let file_path = "./tests/variable/unreached_undefined.lox";
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
#[test]
fn use_nil_as_var() {
    let file_path = "./tests/variable/use_nil_as_var.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    // dbg!(printer.result.borrow());
    assert!(printer.result.borrow()[0].starts_with("Error: Expected variable name"));
}
#[test]
fn duplicate_local() {
    let file_path = "./tests/variable/duplicate_local.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Already a variable with this name in this scope."));

}
#[test]
fn in_nested_block() {
    let file_path = "./tests/variable/in_nested_block.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "outer"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn scope_reuse_in_different_blcks() {
    let file_path = "./tests/variable/scope_reuse_in_different_blocks.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "first", "second"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn undefined_global() {
    let file_path = "./tests/variable/undefined_global.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Undefined variable 'notDefined'"));

}
#[test]
fn use_false_as_var() {
    let file_path = "./tests/variable/use_false_as_var.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Expected variable nam"));

}
#[test]
#[ignore]
fn use_this_as_var() {
    let file_path = "./tests/variable/use_this_as_var.lox";
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
#[ignore = "Duplicate params do not fail right now for some reason. Fix!"]
fn duplicate_parameter() {
    let file_path = "./tests/variable/duplicate_parameter.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Already a variable with this name in this scope"));

}
#[test]
fn local_from_method() {
    let file_path = "./tests/variable/local_from_method.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "variable"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn shadow_and_local() {
    let file_path = "./tests/variable/shadow_and_local.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "outer", "inner"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn undefined_local() {
    let file_path = "./tests/variable/undefined_local.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Undefined variable 'notDefined'"));

}
#[test]
fn use_global_in_initializer() {
    let file_path = "./tests/variable/use_global_in_initializer.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "value"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn early_bound() {
    let file_path = "./tests/variable/early_bound.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "outer", "outer"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn redeclare_global() {
    let file_path = "./tests/variable/redeclare_global.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "Nil"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn shadow_global() {
    let file_path = "./tests/variable/shadow_global.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "shadow", "global"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}

#[test]
fn uninitialized() {
    let file_path = "./tests/variable/uninitialized.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    let result = vec![
        "Nil"
    ];
    // dbg!(printer.result.borrow());
    assert_eq!(*printer.result.borrow(), result);
}
#[test]
fn use_local_in_initializer() {
    let file_path = "./tests/variable/use_local_in_initializer.lox";
    let source_code = fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read file: {}", file_path));
    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(false, &printer);
    lox_runner.run(&source_code);
    assert!(printer.result.borrow()[0].starts_with("Error: Can't read local variables in its own declaration"));

}