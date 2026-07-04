// ============================================================
// TOPIC: Functions  (fn se apna kaam ka block banana)
// ============================================================
// Run:  rustc functions.rs && ./functions
// ------------------------------------------------------------
// Function banane ka syntax:  fn naam(parameters) -> return_type { body }
// Rust me function ka order matter nahi karta -> main ke neeche wale
// functions bhi call ho jate hai.

fn main() {
    // 1) Simple function call (koi parameter nahi, kuch return nahi)
    greet();

    // 2) Function jisme parameter jaata hai
    say_hello("Neha");

    // 3) Function jo value return karta hai
    let total = add(10, 20);
    println!("add(10, 20) = {}", total);

    // 4) Ek function dusre function ko call kar sakta hai
    let avg = average(80, 90);
    println!("average = {}", avg);

    // 5) Function ko variable me store karke call karna (function pointer)
    let operation = multiply;      // function ka naam bina () ke
    println!("multiply via variable = {}", operation(4, 5));
}

// Bina parameter, bina return value ka function
fn greet() {
    println!("Namaste! functions seekh rahe hai 🦀");
}

// Parameter leta hai (name: &str)
fn say_hello(name: &str) {
    println!("Hello, {}!", name);
}

// Do number leta hai aur unka sum return karta hai
// NOTE: aakhri line par semicolon NAHI -> wahi expression return hoti hai
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// Ek function jo andar hi dusra function (add) use karta hai
fn average(a: i32, b: i32) -> i32 {
    let sum = add(a, b);
    sum / 2
}
