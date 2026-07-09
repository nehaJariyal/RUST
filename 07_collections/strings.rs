// ============================================================
// TOPIC: Strings  (working with String and &str)
// ============================================================
// Run:  rustc strings.rs && ./strings
// ------------------------------------------------------------
// String -> on the heap, growable, owned.
// &str   -> string slice, borrowed, fixed.

fn main() {
    // ---------- 1) Creating a String ----------
    let mut s = String::new();       // empty
    s.push_str("Namaste");           // add string
    s.push(' ');                     // add one char
    s.push_str("Duniya");
    println!("s = {}", s);

    let from = String::from("Rust");         // from literal
    let to_owned = "Programming".to_string(); // &str -> String
    println!("{} {}", from, to_owned);

    // ---------- 2) Concatenation ----------
    let hello = String::from("Hello");
    let world = String::from("World");
    // format! is the easiest (no ownership is moved)
    let combined = format!("{}, {}!", hello, world);
    println!("format se = {}", combined);

    // ---------- 3) Length and properties ----------
    println!("length (bytes) = {}", combined.len());
    println!("khali hai? {}", combined.is_empty());
    println!("'World' hai? {}", combined.contains("World"));

    // ---------- 4) Changing case ----------
    println!("Upper = {}", combined.to_uppercase());
    println!("Lower = {}", combined.to_lowercase());

    // ---------- 5) Iterating (chars) ----------
    for c in "abc".chars() {
        println!("char: {}", c);
    }
    println!("Total chars = {}", "namaste".chars().count());

    // ---------- 6) Splitting ----------
    let csv = "neha,amit,priya";
    let names: Vec<&str> = csv.split(',').collect();
    println!("split se = {:?}", names);
    for name in &names {
        println!("Naam: {}", name);
    }

    // ---------- 7) trim (remove leading/trailing spaces) ----------
    let messy = "   spaces around   ";
    println!("trim se = '{}'", messy.trim());

    // ---------- 8) replace ----------
    println!("{}", "main sad hu".replace("sad", "khush"));

    // ---------- 9) starts_with / ends_with ----------
    println!("'Rust' se shuru? {}", combined.starts_with("Hello"));
}
