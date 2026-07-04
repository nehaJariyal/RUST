// ============================================================
// TOPIC: References  (& aur *  -  address aur dereference)
// ============================================================
// Run:  rustc references.rs && ./references
// ------------------------------------------------------------
// Reference = kisi value ka "pata" (address) jo use point karta hai
// bina ownership liye.
//   &x   -> x ka reference banao (borrow)
//   *r   -> reference ke andar ki asli value tak jao (dereference)

fn main() {
    // ---------- 1) Basic reference aur dereference ----------
    let x = 10;
    let r = &x;             // r, x ko point kar raha hai
    println!("x = {}", x);
    println!("r (reference) = {}", r);   // print me Rust khud dereference kar deta hai
    println!("*r (dereference) = {}", *r); // manually * lagakar value

    // Compare karte waqt * ki zarurat padti hai
    if *r == 10 {
        println!("*r sach me 10 ke barabar hai");
    }

    // ---------- 2) Mutable reference se value badalna ----------
    let mut y = 5;
    let m = &mut y; // y ka mutable reference
    *m += 20;       // * se y ki asli value badli
    println!("y ab = {}", y);

    // ---------- 3) Reference function me bhejna (value change) ----------
    let mut count = 0;
    increment(&mut count);
    increment(&mut count);
    println!("count = {}", count);

    // ---------- 4) Reference of reference ----------
    let value = 99;
    let ref1 = &value;
    let ref2 = &ref1;         // reference ka bhi reference
    println!("**ref2 = {}", **ref2); // do baar dereference

    // ---------- 5) Reference kabhi bhi dangling nahi hoti ----------
    // Rust guarantee karta hai ki reference hamesha valid data ko point kare.
    // Isliye niche wala function ERROR dega (uncomment karke dekh sakte ho):
    //   fn dangle() -> &String { let s = String::from("x"); &s }
    //   yaha s function ke saath drop ho jayega -> reference invalid
    println!("References safe hote hai ✅");
}

// mutable reference leta hai aur value badhata hai
fn increment(n: &mut i32) {
    *n += 1;
}
