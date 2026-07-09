// ============================================================
// TOPIC: References  (& and *  -  address and dereference)
// ============================================================
// Run:  rustc references.rs && ./references
// ------------------------------------------------------------
// Reference = a "pointer" to a value that lets you use it
// without taking ownership.
//   &x   -> create a reference to x (borrow)
//   *r   -> go to the actual value inside the reference (dereference)

fn main() {
    // ---------- 1) Basic reference and dereference ----------
    let x = 10;
    let r = &x;             // r points to x
    println!("x = {}", x);
    println!("r (reference) = {}", r);   // Rust auto-dereferences when printing
    println!("*r (dereference) = {}", *r); // manually use * to get the value

    // * is needed when comparing
    if *r == 10 {
        println!("*r sach me 10 ke barabar hai");
    }

    // ---------- 2) Changing a value via mutable reference ----------
    let mut y = 5;
    let m = &mut y; // mutable reference to y
    *m += 20;       // changed y's actual value with *
    println!("y ab = {}", y);

    // ---------- 3) Sending a reference to a function (changing value) ----------
    let mut count = 0;
    increment(&mut count);
    increment(&mut count);
    println!("count = {}", count);

    // ---------- 4) Reference of reference ----------
    let value = 99;
    let ref1 = &value;
    let ref2 = &ref1;         // reference to a reference
    println!("**ref2 = {}", **ref2); // dereference twice

    // ---------- 5) References are never dangling ----------
    // Rust guarantees that a reference always points to valid data.
    // So the function below would ERROR (uncomment to try):
    //   fn dangle() -> &String { let s = String::from("x"); &s }
    //   here s is dropped with the function -> reference would be invalid
    println!("References safe hote hai ✅");
}

// takes a mutable reference and increments the value
fn increment(n: &mut i32) {
    *n += 1;
}
