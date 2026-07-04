// ============================================================
// TOPIC: Slices  (collection ke ek HISSE ka reference)
// ============================================================
// Run:  rustc slices.rs && ./slices
// ------------------------------------------------------------
// Slice = kisi String ya Array ka ek continuous part, bina copy kiye.
// Syntax:  &collection[start..end]   (end shamil NAHI hota)

fn main() {
    // ---------- 1) STRING slice (&str) ----------
    let sentence = String::from("Neha Rust seekh rahi hai");

    // index 0 se 4 tak (5 shamil nahi) -> "Neha"
    let first_word = &sentence[0..4];
    // shortcut: shuru se -> [..4] , aakhir tak -> [5..]
    let same_first = &sentence[..4];
    println!("Pehla shabd = '{}' (aur = '{}')", first_word, same_first);

    // ---------- 2) Function ko slice bhejna ----------
    // &str parameter String aur string literal dono accept kar leta hai
    println!("get_first_word = '{}'", get_first_word(&sentence));
    println!("literal par bhi chalta hai = '{}'", get_first_word("Hello Duniya"));

    // ---------- 3) ARRAY slice (&[i32]) ----------
    let numbers = [10, 20, 30, 40, 50];
    let middle = &numbers[1..4]; // index 1,2,3 -> [20, 30, 40]
    println!("Beech ka slice = {:?}", middle);
    println!("Slice ki length = {}", middle.len());

    // Slice par bhi loop chala sakte hai
    let mut sum = 0;
    for n in middle {
        sum += n;
    }
    println!("Slice ka sum = {}", sum);

    // ---------- 4) Pura array as slice ----------
    let full = &numbers[..]; // pura array ka slice
    println!("Pura slice = {:?}", full);
}

// Pehla shabd return karta hai (space milne tak)
// &str return karte hai taaki original data ka hissa point kare
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // string ko bytes me convert kiya
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {          // b' ' matlab space ka byte
            return &s[0..i];       // space se pehle tak ka slice
        }
    }
    &s[..] // agar space nahi mila to poori string
}
