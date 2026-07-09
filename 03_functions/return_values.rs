// ============================================================
// TOPIC: Return Values  (getting a value back from a function)
// ============================================================
// Run:  rustc return_values.rs && ./return_values
// ------------------------------------------------------------
// Return type is specified with  -> type .
// Two ways to return:
//   1) `return value;`  (explicit, can be used in the middle too)
//   2) last line without semicolon (implicit return)

fn main() {
    // 1) Implicit return (no semicolon)
    println!("square(5) = {}", square(5));

    // 2) Explicit return (with early return)
    println!("check(-3) = {}", check_sign(-3));
    println!("check(7)  = {}", check_sign(7));

    // 3) Returning multiple values -> in a TUPLE
    let (sum, product) = sum_and_product(4, 6);
    println!("sum = {}, product = {}", sum, product);

    // 4) Returning nothing -> unit type ()
    // Such functions have no return type written
    log_message("Program chal raha hai");

    // 5) Using if as a return
    println!("bada(3, 9) = {}", bigger(3, 9));
}

// implicit return: no semicolon on the x*x line
fn square(x: i32) -> i32 {
    x * x
}

// explicit + early return: returns immediately when condition is true
fn check_sign(n: i32) -> &'static str {
    if n < 0 {
        return "Negative"; // returns from here
    }
    if n == 0 {
        return "Zero";
    }
    "Positive" // implicit return
}

// tuple return -> multiple values at once
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// returns no value (no return type written = unit `()`)
fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
}

// if expression itself returns a value
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
