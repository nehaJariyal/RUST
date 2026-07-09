// ============================================================
// PROJECT: Calculator  (a small calculator - all concepts together)
// ============================================================
// Run:  rustc calculator.rs && ./calculator
// ------------------------------------------------------------
// Concepts used: enum, match, Result, error handling, functions.

// Types of operations (enum)
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Performs calculation. Returns Err on divide-by-zero (Result).
fn calculate(a: f64, b: f64, op: &Operation) -> Result<f64, String> {
    match op {
        Operation::Add => Ok(a + b),
        Operation::Subtract => Ok(a - b),
        Operation::Multiply => Ok(a * b),
        Operation::Divide => {
            if b == 0.0 {
                Err(String::from("Zero se divide nahi kar sakte!"))
            } else {
                Ok(a / b)
            }
        }
    }
}

// Prints the result in a nice format
fn print_result(a: f64, b: f64, op: &Operation) {
    // match to get the symbol
    let symbol = match op {
        Operation::Add => "+",
        Operation::Subtract => "-",
        Operation::Multiply => "*",
        Operation::Divide => "/",
    };

    match calculate(a, b, op) {
        Ok(result) => println!("{} {} {} = {}", a, symbol, b, result),
        Err(e) => println!("{} {} {} -> Error: {}", a, symbol, b, e),
    }
}

fn main() {
    println!("===== Rust Calculator =====");

    // Run different operations
    print_result(10.0, 5.0, &Operation::Add);
    print_result(10.0, 5.0, &Operation::Subtract);
    print_result(10.0, 5.0, &Operation::Multiply);
    print_result(10.0, 5.0, &Operation::Divide);

    // Error case: divide by zero
    print_result(8.0, 0.0, &Operation::Divide);

    // Calculations on a small list
    println!("\n----- Batch calculations -----");
    let jobs = [
        (2.0, 3.0, Operation::Add),
        (9.0, 3.0, Operation::Divide),
        (4.0, 4.0, Operation::Multiply),
    ];
    for (a, b, op) in &jobs {
        print_result(*a, *b, op);
    }
}
