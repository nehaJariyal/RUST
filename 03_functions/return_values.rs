// ============================================================
// TOPIC: Return Values  (function se value wapas lena)
// ============================================================
// Run:  rustc return_values.rs && ./return_values
// ------------------------------------------------------------
// Return type ko  -> type  se batate hai.
// Do tarike se return kar sakte hai:
//   1) `return value;`  (explicit, beech me bhi kar sakte hai)
//   2) aakhri line par bina semicolon (implicit return)

fn main() {
    // 1) Implicit return (semicolon nahi)
    println!("square(5) = {}", square(5));

    // 2) Explicit return (early return ke saath)
    println!("check(-3) = {}", check_sign(-3));
    println!("check(7)  = {}", check_sign(7));

    // 3) Multiple values return karna -> TUPLE me
    let (sum, product) = sum_and_product(4, 6);
    println!("sum = {}, product = {}", sum, product);

    // 4) Kuch return na karna -> unit type ()
    // Aise function ka koi return type nahi likhte
    log_message("Program chal raha hai");

    // 5) if ko return ki tarah use karna
    println!("bada(3, 9) = {}", bigger(3, 9));
}

// implicit return: a*a wali line par semicolon nahi
fn square(x: i32) -> i32 {
    x * x
}

// explicit + early return: condition true hote hi function se bahar
fn check_sign(n: i32) -> &'static str {
    if n < 0 {
        return "Negative"; // yahi se return ho jayega
    }
    if n == 0 {
        return "Zero";
    }
    "Positive" // implicit return
}

// tuple return -> ek se zyada value ek saath
fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

// koi value return nahi (return type nahi likha = unit `()`)
fn log_message(msg: &str) {
    println!("[LOG] {}", msg);
}

// if expression khud value return karta hai
fn bigger(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
