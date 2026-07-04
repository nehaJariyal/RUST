// ============================================================
// TOPIC: Result<T, E>  (kaam safal hua ya fail - error handling)
// ============================================================
// Run:  rustc result.rs && ./result
// ------------------------------------------------------------
// Result ek enum hai jo batata hai kaam safal hua ya error aaya:
//   enum Result<T, E> { Ok(T), Err(E) }
//   Ok(value) -> safalta, value ke saath
//   Err(error) -> galti, error ke saath

fn main() {
    // ---------- 1) Result return karne wala function ----------
    // Divide by zero se bachne ke liye Result use kiya
    match divide(10.0, 2.0) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match divide(5.0, 0.0) {
        Ok(result) => println!("5 / 0 = {}", result),
        Err(e) => println!("Error: {}", e), // ye chalega
    }

    // ---------- 2) unwrap_or -> Err ho to default ----------
    println!("Safe result = {}", divide(8.0, 0.0).unwrap_or(-1.0));

    // ---------- 3) is_ok / is_err ----------
    let r = divide(9.0, 3.0);
    println!("is_ok? {}", r.is_ok());

    // ---------- 4) String ko number me parse karna (Result deta hai) ----------
    let good = "42".parse::<i32>();   // Ok(42)
    let bad = "abc".parse::<i32>();   // Err(...)
    println!("'42' parse -> {:?}", good);
    println!("'abc' parse -> {:?}", bad);

    // ---------- 5) match se dono handle karna ----------
    match "100".parse::<i32>() {
        Ok(n) => println!("Number mila: {}", n),
        Err(_) => println!("Number nahi tha"),
    }
}

// Result return karta hai: Ok(number) ya Err(message)
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Zero se divide nahi kar sakte"))
    } else {
        Ok(a / b)
    }
}
