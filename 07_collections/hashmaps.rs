// ============================================================
// TOPIC: HashMap<K, V>  (key -> value pairs, like a dictionary)
// ============================================================
// Run:  rustc hashmaps.rs && ./hashmaps
// ------------------------------------------------------------
// HashMap stores values by key (like a phone book:
// name -> number). Order is NOT guaranteed.

// Must import HashMap from std library
use std::collections::HashMap;

fn main() {
    // ---------- 1) Creating a HashMap and inserting ----------
    let mut marks: HashMap<String, i32> = HashMap::new();
    marks.insert(String::from("Neha"), 90);
    marks.insert(String::from("Amit"), 75);
    marks.insert(String::from("Priya"), 85);
    println!("marks = {:?}", marks);

    // ---------- 2) Getting a value (.get -> Option) ----------
    match marks.get("Neha") {
        Some(m) => println!("Neha ke marks = {}", m),
        None => println!("Neha nahi mili"),
    }
    // with default
    let amit = marks.get("Amit").copied().unwrap_or(0);
    println!("Amit = {}", amit);

    // ---------- 3) Updating ----------
    marks.insert(String::from("Neha"), 95); // same key -> value overwritten
    println!("Neha update = {:?}", marks.get("Neha"));

    // ---------- 4) entry -> insert only if not present ----------
    marks.entry(String::from("Rahul")).or_insert(60); // Rahul will be added
    marks.entry(String::from("Neha")).or_insert(0);    // Neha already exists, nothing happens
    println!("entry ke baad Rahul = {:?}", marks.get("Rahul"));

    // ---------- 5) Iterating (keys + values) ----------
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
        // if word appears for the first time, start at 0, then +1
        *count.entry(word).or_insert(0) += 1;
    }
    println!("Word count = {:?}", count);
}
