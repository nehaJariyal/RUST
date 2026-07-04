// ============================================================
// PROJECT: Calculator  (ek chhota calculator - saare concepts ek saath)
// ============================================================
// Run:  rustc calculator.rs && ./calculator
// ------------------------------------------------------------
// Is project me use kiya: enum, match, Result, error handling, functions.

// Operation ke types (enum)
#[derive(Debug)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

// Calculation karta hai. Divide-by-zero par Err deta hai (Result).
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

// Sundar tarike se result print karta hai
fn print_result(a: f64, b: f64, op: &Operation) {
    // symbol nikalne ke liye match
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

    // Alag alag operations chalate hai
    print_result(10.0, 5.0, &Operation::Add);
    print_result(10.0, 5.0, &Operation::Subtract);
    print_result(10.0, 5.0, &Operation::Multiply);
    print_result(10.0, 5.0, &Operation::Divide);

    // Error case: divide by zero
    print_result(8.0, 0.0, &Operation::Divide);

    // Ek chhoti list par calculations
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
