// ============================================================
// TOPIC: Variables in Rust  (Rust me variables kaise banate hai)
// ============================================================
// Is file ko run karne ke liye:  rustc variables.rs && ./variables
// ------------------------------------------------------------

fn main() {
    // 1) IMMUTABLE variable (default)
    // Rust me variable by default IMMUTABLE hota hai matlab ek baar
    // value set kar di to badal nahi sakte.
    let name = "neha";
    println!("Immutable variable name = {}", name);

    // name = "harpreet";   // <-- ye line uncomment karoge to ERROR aayega
                            //     kyunki `name` mutable nahi hai.

    // 2) MUTABLE variable
    // Agar value change karni ho to `mut` keyword lagate hai.
    let mut age = 20;
    println!("Pehle age = {}", age);
    age = 21;                      // ab change kar sakte hai kyunki `mut` laga hai
    println!("Badalne ke baad age = {}", age);

    // 3) SHADOWING
    // Same naam ka variable dubara `let` se banate hai -> purana chhup jata hai.
    // Isse hum type bhi change kar sakte hai (yaha number -> string).
    let score = 5;                 // score ek integer hai
    let score = score + 1;         // naya score = 6 (purana wala shadow ho gaya)
    let score = format!("Score is {}", score); // ab score ek String ban gaya
    println!("{}", score);

    // 4) TYPE ANNOTATION (type khud batana)
    // Rust khud type guess kar leta hai, par hum manually bhi bata sakte hai.
    let marks: u32 = 90;           // u32 = unsigned 32-bit integer (sirf +ve)
    let pi: f64 = 3.14;            // f64 = 64-bit floating point (decimal)
    println!("marks = {}, pi = {}", marks, pi);

    // 5) MULTIPLE variables ek saath (tuple destructuring)
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 6) CONSTANT vs variable ka chhota jhalak
    // (detail me constants.rs file me padhna)
    let _unused = 100; // `_` lagane se "unused variable" warning nahi aati
}
