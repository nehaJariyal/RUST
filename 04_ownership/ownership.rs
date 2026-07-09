// ============================================================
// TOPIC: Ownership  (Rust's MOST important concept)
// ============================================================
// Run:  rustc ownership.rs && ./ownership
// ------------------------------------------------------------
// The 3 rules of ownership:
//   1) Every value has one OWNER.
//   2) There can be only ONE owner at a time.
//   3) When the owner goes out of scope -> value is removed from memory (drop).
// This is why Rust doesn't need a garbage collector.

fn main() {
    // ---------- 1) The concept of SCOPE ----------
    {
        let s = String::from("scope ke andar");
        println!("{}", s);
    } // <- here `s`'s scope ends, memory is automatically freed
    // println!("{}", s); // <-- ERROR: s no longer exists

    // ---------- 2) MOVE (changing owner of heap data) ----------
    // String is built on the heap. When you assign it to another variable,
    // ownership "moves", and the old variable becomes invalid.
    let s1 = String::from("hello");
    let s2 = s1; // ownership MOVED from s1 -> s2
    // println!("{}", s1); // <-- ERROR: s1 is no longer valid (moved)
    println!("s2 = {}", s2); // s2 is valid

    // ---------- 3) CLONE (making a real copy) ----------
    // If you need both, use .clone() to make a full copy of heap data.
    let a = String::from("duplicate");
    let b = a.clone(); // now a and b each own their own separate value
    println!("a = {}, b = {}", a, b);

    // ---------- 4) COPY types (on the stack, no move) ----------
    // i32, f64, bool, char, and tuples of these are "Copy".
    // They get a simple copy instead of a move -> both stay valid.
    let x = 5;
    let y = x; // copied, not moved
    println!("x = {}, y = {} (dono valid)", x, y);

    // ---------- 5) Giving a value to a function = ownership move ----------
    let text = String::from("main function ko jaunga");
    take_ownership(text); // ownership moved into the function
    // println!("{}", text); // <-- ERROR: text has been moved

    // ---------- 6) Getting ownership back (via return) ----------
    let given = String::from("data");
    let returned = give_back(given); // went in, came back out
    println!("wapas mila: {}", returned);
}

// This function becomes the owner of the String; when the function ends, it's dropped
fn take_ownership(s: String) {
    println!("take_ownership ke andar: {}", s);
} // s is dropped here

// Takes ownership inside and returns it back to the caller
fn give_back(s: String) -> String {
    s // ownership goes back to the caller
}
