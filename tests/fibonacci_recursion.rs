use rlox::lox::{Lox, printer::TestPrinter};

#[test]
fn test_recursive_fibonacci_seq() {
    let fib_code = String::from("
    var counter = 0;
    fun fib(n) {
       counter = counter + 1;
      if (n <= 1) return n;
      return fib(n - 2) + fib(n - 1);
    }
    for (var i = 0; i < 20; i = i + 1) {
      print fib(i);
    }
    ");

    let printer = TestPrinter::default();
    let mut lox_runner = Lox::new(true, &printer);

    lox_runner.run(&fib_code);
    let result = vec![
        "0", "1", "1", "2", "3", "5", "8", "13", "21", "34", "55", "89", "144", "233", "377", "610", "987", "1597", "2584", "4181"
        ];

    assert_eq!(*printer.result.borrow(), result);
}