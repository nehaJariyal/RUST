// ============================================================
// TOPIC: Result error handling  (handling recoverable errors)
// ============================================================
// Run:  rustc result.rs && ./result
// ------------------------------------------------------------
// With Result<T, E> we can "handle" errors without crashing.
//   Ok(T)  -> success
//   Err(E) -> failure

fn main() {
    // ---------- 1) Handling with match (most basic) ----------
    match parse_age("25") {
        Ok(age) => println!("Age = {}", age),
        Err(e) => println!("Galti: {}", e),
    }
    match parse_age("abc") {
        Ok(age) => println!("Age = {}", age),
        Err(e) => println!("Galti: {}", e), // this will run
    }

    // ---------- 2) unwrap_or -> default on Err ----------
    println!("Default se = {}", parse_age("xyz").unwrap_or(18));

    // ---------- 3) unwrap_or_else -> run a closure on Err ----------
    let age = parse_age("bad").unwrap_or_else(|err| {
        println!("(error tha: {}) -> 0 use kar rahe", err);
        0
    });
    println!("age = {}", age);

    // ---------- 4) Transform with map / map_err ----------
    let doubled = parse_age("10").map(|a| a * 2); // Ok(20)
    println!("map se = {:?}", doubled);

    // ---------- 5) Result of multiple operations ----------
    match safe_divide(10, 2) {
        Ok(r) => println!("10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
    match safe_divide(10, 0) {
        Ok(r) => println!("10 / 0 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
}

// Converts a String to age (u32)
fn parse_age(input: &str) -> Result<u32, String> {
    match input.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' ek valid number nahi hai", input)),
    }
}

// Integer division that avoids dividing by zero
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Zero se divide mana hai"));
    }
    Ok(a / b)
}
