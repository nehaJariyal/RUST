// ============================================================
// TOPIC: Functions  (creating reusable blocks of work with fn)
// ============================================================
// Run:  rustc functions.rs && ./functions
// ------------------------------------------------------------
// Syntax for creating a function:  fn name(parameters) -> return_type { body }
// In Rust, function order doesn't matter -> functions defined below main
// can still be called.

fn main() {
    // 1) Simple function call (no parameters, no return value)
    greet();

    // 2) Function that takes parameters
    say_hello("Neha");

    // 3) Function that returns a value
    let total = add(10, 20);
    println!("add(10, 20) = {}", total);

    // 4) One function can call another function
    let avg = average(80, 90);
    println!("average = {}", avg);

    // 5) Store a function in a variable and call it (function pointer)
    let operation = multiply;      // function name without ()
    println!("multiply via variable = {}", operation(4, 5));
}

// Function with no parameters and no return value
fn greet() {
    println!("Namaste! functions seekh rahe hai 🦀");
}

// Takes a parameter (name: &str)
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

// Takes two numbers and returns their sum
// NOTE: no semicolon on the last line -> that expression is returned
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// A function that uses another function (add) internally
fn average(a: i32, b: i32) -> i32 {
    let sum = add(a, b);
    sum / 2
}
