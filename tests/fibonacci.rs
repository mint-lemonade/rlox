use rlox::lox::{Lox, printer::TestPrinter};

#[test]
fn test_fibonacci_seq() {
    let fib_code = String::from("
      var a = 0;
      var temp;
       
       for (var b = 1; a < 10000; b = temp + b) {
         print a;
         temp = a;
         a = b;
       }
    ");

    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(true, &printer);

    lox_runner.run(&fib_code);
    let result = vec![
        "0", "1", "1", "2", "3", "5", "8", "13", "21", "34", "55", "89", "144", "233", "377", "610", "987", "1597", "2584", "4181", "6765"
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();

    assert_eq!(*printer.result.borrow(), result);
}