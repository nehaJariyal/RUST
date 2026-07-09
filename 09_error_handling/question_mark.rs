// ============================================================
// TOPIC: ? operator  (shortcut for propagating errors)
// ============================================================
// Run:  rustc question_mark.rs && ./question_mark
// ------------------------------------------------------------
// What the `?` operator does:
//   * if Result is Ok(v) -> unwraps v and continues
//   * if Err(e)          -> immediately RETURNS that error
// This avoids writing match over and over. Very clean code.

fn main() {
    // A function using ? must itself return Result,
    // so in main we match/print its result.
    match calculate("10", "2") {
        Ok(result) => println!("10 + 2 phir /2... = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Here "abc" will fail to parse -> ? will immediately return Err
    match calculate("abc", "2") {
        Ok(result) => println!("Result = {}", result),
        Err(e) => println!("Error: {}", e), // this will run
    }

    // Here divide by zero
    match calculate("10", "0") {
        Ok(result) => println!("Result = {}", result),
        Err(e) => println!("Error: {}", e), // this will run
    }
}

// Takes two strings, parses both, adds them, then divides by 2.
// Each step returns Result -> `?` automatically propagates errors.
fn calculate(a: &str, b: &str) -> Result<i32, String> {
    // parse returns Result. If Err, returns from here.
    let x = parse_number(a)?; // <-- the magic of ?
    let y = parse_number(b)?;

    let sum = x + y;
    let result = divide(sum, 2)?; // divide also returns Result
    Ok(result)
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{}' number nahi hai", s))
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Zero se divide nahi"))
    } else {
        Ok(a / b)
    }
}
