// ============================================================
// TOPIC: Result error handling  (recoverable errors sambhalna)
// ============================================================
// Run:  rustc result.rs && ./result
// ------------------------------------------------------------
// Result<T, E> se hum error ko "sambhal" sakte hai bina crash kiye.
//   Ok(T)  -> safal
//   Err(E) -> galti

fn main() {
    // ---------- 1) match se handle karna (sabse basic) ----------
    match parse_age("25") {
        Ok(age) => println!("Age = {}", age),
        Err(e) => println!("Galti: {}", e),
    }
    match parse_age("abc") {
        Ok(age) => println!("Age = {}", age),
        Err(e) => println!("Galti: {}", e), // ye chalega
    }

    // ---------- 2) unwrap_or -> Err par default ----------
    println!("Default se = {}", parse_age("xyz").unwrap_or(18));

    // ---------- 3) unwrap_or_else -> Err par closure chalao ----------
    let age = parse_age("bad").unwrap_or_else(|err| {
        println!("(error tha: {}) -> 0 use kar rahe", err);
        0
    });
    println!("age = {}", age);

    // ---------- 4) map / map_err se transform ----------
    let doubled = parse_age("10").map(|a| a * 2); // Ok(20)
    println!("map se = {:?}", doubled);

    // ---------- 5) Multiple operations ka result ----------
    match safe_divide(10, 2) {
        Ok(r) => println!("10 / 2 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
    match safe_divide(10, 0) {
        Ok(r) => println!("10 / 0 = {}", r),
        Err(e) => println!("Error: {}", e),
    }
}

// String ko age (u32) me convert karta hai
fn parse_age(input: &str) -> Result<u32, String> {
    match input.parse::<u32>() {
        Ok(n) => Ok(n),
        Err(_) => Err(format!("'{}' ek valid number nahi hai", input)),
    }
}

// Integer division jo zero se bachta hai
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err(String::from("Zero se divide mana hai"));
    }
    Ok(a / b)
}
