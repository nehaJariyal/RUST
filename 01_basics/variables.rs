// ============================================================
// TOPIC: Variables in Rust  (how to create variables in Rust)
// ============================================================
// To run this file:  rustc variables.rs && ./variables
// ------------------------------------------------------------

fn main() {
    // 1) IMMUTABLE variable (default)
    // In Rust, variables are IMMUTABLE by default, meaning once you
    // set a value you cannot change it.
    let name = "neha";
    println!("Immutable variable name = {}", name);

    // name = "harpreet";   // <-- uncomment this line and you'll get an ERROR
                            //     because `name` is not mutable.

    // 2) MUTABLE variable
    // If you need to change the value, use the `mut` keyword.
    let mut age = 20;
    println!("Pehle age = {}", age);
    age = 21;                      // can change now because `mut` is used
    println!("Badalne ke baad age = {}", age);

    // 3) SHADOWING
    // Create a variable with the same name again using `let` -> the old one is hidden.
    // This also lets you change the type (here number -> string).
    let score = 5;                 // score is an integer
    let score = score + 1;         // new score = 6 (old one was shadowed)
    let score = format!("Score is {}", score); // now score is a String
    println!("{}", score);

    // 4) TYPE ANNOTATION (specify the type yourself)
    // Rust can infer types, but you can also specify them manually.
    let marks: u32 = 90;           // u32 = unsigned 32-bit integer (positive only)
    let pi: f64 = 3.14;            // f64 = 64-bit floating point (decimal)
    println!("marks = {}, pi = {}", marks, pi);

    // 5) MULTIPLE variables at once (tuple destructuring)
    let (x, y, z) = (1, 2, 3);
    println!("x = {}, y = {}, z = {}", x, y, z);

    // 6) A quick glimpse of CONSTANT vs variable
    // (read constants.rs for details)
    let _unused = 100; // prefixing with `_` avoids "unused variable" warning
}
