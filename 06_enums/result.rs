// ============================================================
// TOPIC: Result<T, E>  (success or failure - error handling)
// ============================================================
// Run:  rustc result.rs && ./result
// ------------------------------------------------------------
// Result is an enum that tells whether an operation succeeded or failed:
//   enum Result<T, E> { Ok(T), Err(E) }
//   Ok(value) -> success, with a value
//   Err(error) -> failure, with an error

fn main() {
    // ---------- 1) Function that returns Result ----------
    // Used Result to avoid divide by zero
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(5.0, 0.0) {
        Ok(result) => println!("5 / 0 = {}", result),
        Err(e) => println!("Error: {}", e), // this will run
    }

    // ---------- 2) unwrap_or -> default when Err ----------
    println!("Safe result = {}", divide(8.0, 0.0).unwrap_or(-1.0));

    // ---------- 3) is_ok / is_err ----------
    let r = divide(9.0, 3.0);
    println!("is_ok? {}", r.is_ok());

    // ---------- 4) Parsing a String into a number (returns Result) ----------
    let good = "42".parse::<i32>();   // Ok(42)
    let bad = "abc".parse::<i32>();   // Err(...)
    println!("'42' parse -> {:?}", good);
    println!("'abc' parse -> {:?}", bad);

    // ---------- 5) Handling both cases with match ----------
    match "100".parse::<i32>() {
        Ok(n) => println!("Number mila: {}", n),
        Err(_) => println!("Number nahi tha"),
    }
}

// Returns Result: Ok(number) or Err(message)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Zero se divide nahi kar sakte"))
    } else {
        Ok(a / b)
    }
}
