// ============================================================
// TOPIC: Data Types in Rust  (Rust ke basic data types)
// ============================================================
// Run:  rustc data_types.rs && ./data_types
// ------------------------------------------------------------
// Rust ek "statically typed" language hai -> har value ka type
// compile time par pata hona chahiye.
// Types do tarah ke hote hai: Scalar (single value) aur Compound (multiple).

fn main() {
    // ---------- 1) INTEGERS (poore number) ----------
    // Signed   -> i8, i16, i32, i64, i128  (+ve aur -ve dono)
    // Unsigned -> u8, u16, u32, u64, u128  (sirf +ve)
    let a: i32 = -50;      // default integer type i32 hota hai
    let b: u8 = 255;       // u8 ki max value 255 hoti hai
    println!("Integers -> a = {}, b = {}", a, b);

    // ---------- 2) FLOATS (decimal number) ----------
    // f32 (32-bit) aur f64 (64-bit, default aur zyada precise)
    let pi: f64 = 3.14159;
    let temp: f32 = 36.6;
    println!("Floats -> pi = {}, temp = {}", pi, temp);

    // ---------- 3) BOOLEAN (sach/jhoot) ----------
    let is_rust_fun: bool = true;
    let is_hard: bool = false;
    println!("Boolean -> fun? {}, hard? {}", is_rust_fun, is_hard);

    // ---------- 4) CHARACTER (single character) ----------
    // char single quotes '' me hota hai aur emoji/unicode bhi rakh sakta hai
    let letter: char = 'N';
    let emoji: char = '🦀'; // Rust ka mascot crab
    println!("Char -> {} {}", letter, emoji);

    // ---------- 5) TUPLE (alag alag type ek saath) ----------
    // Fixed length hoti hai. Har value ka apna type ho sakta hai.
    let person: (&str, i32, f64) = ("Neha", 21, 5.4);
    println!("Tuple -> naam: {}, age: {}, height: {}", person.0, person.1, person.2);

    // Tuple ko todkar (destructure) alag variables me daal sakte hai
    let (naam, age, height) = person;
    println!("Destructured -> {} {} {}", naam, age, height);

    // ---------- 6) ARRAY (same type, fixed size) ----------
    // [type ; kitne elements]
    let marks: [i32; 5] = [90, 85, 70, 95, 60];
    println!("Array pura -> {:?}", marks);      // {:?} debug print
    println!("Pehla mark -> {}", marks[0]);     // index 0 se start
    println!("Array ki length -> {}", marks.len());

    // ---------- 7) STRING types ----------
    // &str  -> string slice (fixed, borrow kiya hua)
    // String -> heap par bani, grow/change ho sakti hai
    let fixed: &str = "main fixed hu";
    let mut growable: String = String::from("main badal");
    growable.push_str(" sakti hu");
    println!("&str  -> {}", fixed);
    println!("String -> {}", growable);
}
