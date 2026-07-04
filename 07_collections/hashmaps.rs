// ============================================================
// TOPIC: HashMap<K, V>  (key -> value ki jodi, dictionary jaisa)
// ============================================================
// Run:  rustc hashmaps.rs && ./hashmaps
// ------------------------------------------------------------
// HashMap key ke through value store karta hai (jaise phone book:
// naam -> number). Order guaranteed NAHI hota.

// std ki library se HashMap import karna padta hai
use std::collections::HashMap;

fn main() {
    // ---------- 1) HashMap banana aur insert ----------
    let mut marks: HashMap<String, i32> = HashMap::new();
    marks.insert(String::from("Neha"), 90);
    marks.insert(String::from("Amit"), 75);
    marks.insert(String::from("Priya"), 85);
    println!("marks = {:?}", marks);

    // ---------- 2) Value nikalna (.get -> Option) ----------
    match marks.get("Neha") {
        Some(m) => println!("Neha ke marks = {}", m),
        None => println!("Neha nahi mili"),
    }
    // default ke saath
    let amit = marks.get("Amit").copied().unwrap_or(0);
    println!("Amit = {}", amit);

    // ---------- 3) Update karna ----------
    marks.insert(String::from("Neha"), 95); // same key -> value overwrite
    println!("Neha update = {:?}", marks.get("Neha"));

    // ---------- 4) entry -> nahi hai to hi insert karo ----------
    marks.entry(String::from("Rahul")).or_insert(60); // Rahul add hoga
    marks.entry(String::from("Neha")).or_insert(0);    // Neha already hai, kuch nahi
    println!("entry ke baad Rahul = {:?}", marks.get("Rahul"));

    // ---------- 5) Ghumna (keys + values) ----------
    for (name, mark) in &marks {
        println!("{} -> {}", name, mark);
    }

    // ---------- 6) Useful methods ----------
    println!("Kitne students? {}", marks.len());
    println!("Amit hai? {}", marks.contains_key("Amit"));
    marks.remove("Amit");
    println!("Amit hatane ke baad hai? {}", marks.contains_key("Amit"));

    // ---------- 7) Counting example (word frequency) ----------
    let text = "aa bb aa cc bb aa";
    let mut count: HashMap<&str, i32> = HashMap::new();
    for word in text.split(' ') {
        // agar word pehli baar aaya to 0 se shuru, phir +1
        *count.entry(word).or_insert(0) += 1;
    }
    println!("Word count = {:?}", count);
}
