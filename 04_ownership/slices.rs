// ============================================================
// TOPIC: Slices  (a reference to a PART of a collection)
// ============================================================
// Run:  rustc slices.rs && ./slices
// ------------------------------------------------------------
// Slice = a continuous part of a String or Array, without copying.
// Syntax:  &collection[start..end]   (end is NOT included)

fn main() {
    // ---------- 1) STRING slice (&str) ----------
    let sentence = String::from("Neha Rust seekh rahi hai");

    // index 0 to 4 (5 not included) -> "Neha"
    let first_word = &sentence[0..4];
    // shortcuts: from start -> [..4] , to end -> [5..]
    let same_first = &sentence[..4];
    println!("Pehla shabd = '{}' (aur = '{}')", first_word, same_first);

    // ---------- 2) Passing a slice to a function ----------
    // &str parameter accepts both String and string literals
    println!("get_first_word = '{}'", get_first_word(&sentence));
    println!("literal par bhi chalta hai = '{}'", get_first_word("Hello Duniya"));

    // ---------- 3) ARRAY slice (&[i32]) ----------
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4]; // index 1,2,3 -> [20, 30, 40]
    println!("Beech ka slice = {:?}", middle);
    println!("Slice ki length = {}", middle.len());

    // You can also loop over a slice
    let mut sum = 0;
    for n in middle {
        sum += n;
    }
    println!("Slice ka sum = {}", sum);

    // ---------- 4) Entire array as a slice ----------
    let full = &numbers[..]; // slice of the whole array
    println!("Pura slice = {:?}", full);
}

// Returns the first word (until a space is found)
// Returns &str so it points to a part of the original data
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // converted string to bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {          // b' ' means the byte for space
            return &s[0..i];       // slice up to the space
        }
    }
    &s[..] // if no space found, return the whole string
}
