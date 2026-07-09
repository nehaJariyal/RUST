// ============================================================
// TOPIC: Data Types in Rust  (Rust's basic data types)
// ============================================================
// Run:  rustc data_types.rs && ./data_types
// ------------------------------------------------------------
// Rust is a "statically typed" language -> every value's type must
// be known at compile time.
// Types come in two kinds: Scalar (single value) and Compound (multiple).

fn main() {
    // ---------- 1) INTEGERS (whole numbers) ----------
    // Signed   -> i8, i16, i32, i64, i128  (both positive and negative)
    // Unsigned -> u8, u16, u32, u64, u128  (positive only)
    let a: i32 = -50;      // default integer type is i32
    let b: u8 = 255;       // max value of u8 is 255
    println!("Integers -> a = {}, b = {}", a, b);

    // ---------- 2) FLOATS (decimal numbers) ----------
    // f32 (32-bit) and f64 (64-bit, default and more precise)
    let pi: f64 = 3.14159;
    let temp: f32 = 36.6;
    println!("Floats -> pi = {}, temp = {}", pi, temp);

    // ---------- 3) BOOLEAN (true/false) ----------
    let is_rust_fun: bool = true;
    let is_hard: bool = false;
    println!("Boolean -> fun? {}, hard? {}", is_rust_fun, is_hard);

    // ---------- 4) CHARACTER (single character) ----------
    // char goes in single quotes '' and can hold emoji/unicode too
    let letter: char = 'N';
    let emoji: char = '🦀'; // Rust's mascot crab
    println!("Char -> {} {}", letter, emoji);

    // ---------- 5) TUPLE (different types together) ----------
    // Fixed length. Each value can have its own type.
    let person: (&str, i32, f64) = ("Neha", 21, 5.4);
    println!("Tuple -> naam: {}, age: {}, height: {}", person.0, person.1, person.2);

    // You can destructure a tuple into separate variables
    let (naam, age, height) = person;
    println!("Destructured -> {} {} {}", naam, age, height);

    // ---------- 6) ARRAY (same type, fixed size) ----------
    // [type ; number of elements]
    let marks: [i32; 5] = [90, 85, 70, 95, 60];
    println!("Array pura -> {:?}", marks);      // {:?} debug print
    println!("Pehla mark -> {}", marks[0]);     // index starts at 0
    println!("Array ki length -> {}", marks.len());

    // ---------- 7) STRING types ----------
    // &str  -> string slice (fixed, borrowed)
    // String -> built on the heap, can grow/change
    let fixed: &str = "main fixed hu";
    let mut growable: String = String::from("main badal");
    growable.push_str(" sakti hu");
    println!("&str  -> {}", fixed);
    println!("String -> {}", growable);
}
