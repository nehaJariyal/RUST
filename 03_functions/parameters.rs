// ============================================================
// TOPIC: Parameters  (function ko data dene ke tarike)
// ============================================================
// Run:  rustc parameters.rs && ./parameters
// ------------------------------------------------------------
// Har parameter ka TYPE batana Rust me zaruri hai.

fn main() {
    // 1) Ek se zyada parameter, alag alag type
    print_info("Neha", 21);

    // 2) By VALUE bhejna (copy hone wale types - i32, f64, bool, char)
    let n = 5;
    double(n);
    println!("main me n abhi bhi = {} (copy gaya tha)", n);

    // 3) By REFERENCE bhejna (&) -> bade data ko copy kiye bina bhejna
    let name = String::from("Rust");
    print_length(&name); // & lagaya -> ownership nahi gaya
    println!("main me name abhi bhi use ho sakta = {}", name);

    // 4) MUTABLE reference (&mut) -> function value ko badal sakta hai
    let mut score = 10;
    add_ten(&mut score); // &mut -> asli variable badlega
    println!("score badalne ke baad = {}", score);

    // 5) Array/slice ko parameter ki tarah bhejna
    let marks = [90, 80, 70];
    println!("Total marks = {}", sum_all(&marks));
}

// do parameter: name (&str) aur age (u32)
fn print_info(name: &str, age: u32) {
    println!("Naam: {}, Umar: {}", name, age);
}

// by value -> `x` yaha ek copy hai
fn double(x: i32) {
    println!("Andar double = {}", x * 2);
}

// by reference -> sirf padhne ke liye borrow kiya
fn print_length(text: &String) {
    println!("'{}' ki length = {}", text, text.len());
}

// mutable reference -> asli value badal sakte hai
fn add_ten(value: &mut i32) {
    *value += 10; // * se reference ke andar ki value tak pahunchte hai (dereference)
}

// slice parameter (&[i32]) -> array ya vector dono chal jate hai
fn sum_all(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
