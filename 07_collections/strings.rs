// ============================================================
// TOPIC: Strings  (String aur &str ke saath kaam)
// ============================================================
// Run:  rustc strings.rs && ./strings
// ------------------------------------------------------------
// String -> heap par, growable, owned.
// &str   -> string slice, borrow kiya hua, fixed.

fn main() {
    // ---------- 1) String banana ----------
    let mut s = String::new();       // khali
    s.push_str("Namaste");           // string add
    s.push(' ');                     // ek char add
    s.push_str("Duniya");
    println!("s = {}", s);

    let from = String::from("Rust");         // literal se
    let to_owned = "Programming".to_string(); // &str -> String
    println!("{} {}", from, to_owned);

    // ---------- 2) Jodna (concatenation) ----------
    let hello = String::from("Hello");
    let world = String::from("World");
    // format! sabse aasan (kisi ka ownership nahi jaata)
    let combined = format!("{}, {}!", hello, world);
    println!("format se = {}", combined);

    // ---------- 3) Length aur properties ----------
    println!("length (bytes) = {}", combined.len());
    println!("khali hai? {}", combined.is_empty());
    println!("'World' hai? {}", combined.contains("World"));

    // ---------- 4) Case badalna ----------
    println!("Upper = {}", combined.to_uppercase());
    println!("Lower = {}", combined.to_lowercase());

    // ---------- 5) Ghumna (chars) ----------
    for c in "abc".chars() {
        println!("char: {}", c);
    }
    println!("Total chars = {}", "namaste".chars().count());

    // ---------- 6) Split karna ----------
    let csv = "neha,amit,priya";
    let names: Vec<&str> = csv.split(',').collect();
    println!("split se = {:?}", names);
    for name in &names {
        println!("Naam: {}", name);
    }

    // ---------- 7) trim (aage-peeche ki space hatana) ----------
    let messy = "   spaces around   ";
    println!("trim se = '{}'", messy.trim());

    // ---------- 8) replace ----------
    println!("{}", "main sad hu".replace("sad", "khush"));

    // ---------- 9) starts_with / ends_with ----------
    println!("'Rust' se shuru? {}", combined.starts_with("Hello"));
}
