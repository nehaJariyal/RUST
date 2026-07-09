// ============================================================
// TOPIC: Parameters  (ways to pass data to a function)
// ============================================================
// Run:  rustc parameters.rs && ./parameters
// ------------------------------------------------------------
// Every parameter's TYPE must be specified in Rust.

fn main() {
    // 1) Multiple parameters, different types
    print_info("Neha", 21);

    // 2) Passing by VALUE (Copy types - i32, f64, bool, char)
    let n = 5;
    double(n);
    println!("main me n abhi bhi = {} (copy gaya tha)", n);

    // 3) Passing by REFERENCE (&) -> send large data without copying
    let name = String::from("Rust");
    print_length(&name); // & used -> ownership did not move
    println!("main me name abhi bhi use ho sakta = {}", name);

    // 4) MUTABLE reference (&mut) -> function can change the value
    let mut score = 10;
    add_ten(&mut score); // &mut -> the original variable will change
    println!("score badalne ke baad = {}", score);

    // 5) Passing an array/slice as a parameter
    let marks = [90, 80, 70];
    println!("Total marks = {}", sum_all(&marks));
}

// two parameters: name (&str) and age (u32)
fn print_info(name: &str, age: u32) {
    println!("Naam: {}, Umar: {}", name, age);
}

// by value -> `x` here is a copy
fn double(x: i32) {
    println!("Andar double = {}", x * 2);
}

// by reference -> borrowed only for reading
fn print_length(text: &String) {
    println!("'{}' ki length = {}", text, text.len());
}

// mutable reference -> can change the original value
fn add_ten(value: &mut i32) {
    *value += 10; // * dereferences to reach the value inside the reference
}

// slice parameter (&[i32]) -> works with both array and vector
fn sum_all(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
