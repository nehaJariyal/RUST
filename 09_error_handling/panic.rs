// ============================================================
// TOPIC: panic!  (program ko turant rok dena - unrecoverable error)
// ============================================================
// Run:  rustc panic.rs && ./panic
// ------------------------------------------------------------
// Rust me 2 tarah ke error hote hai:
//   1) Recoverable   -> Result<T, E> (sambhal sakte hai)
//   2) Unrecoverable -> panic! (program crash ho jaata hai)
// panic tab use karo jab aage badhna safe hi na ho.

fn main() {
    // ---------- 1) unwrap -> Some/Ok par value, warna PANIC ----------
    let good: Option<i32> = Some(10);
    println!("unwrap Some = {}", good.unwrap()); // 10 milega

    // NOTE: neeche wali line PANIC karti (isliye comment ki hui hai):
    // let bad: Option<i32> = None;
    // bad.unwrap(); // panic: called `Option::unwrap()` on a `None`

    // ---------- 2) expect -> unwrap jaisa par custom message ----------
    let value: Result<i32, String> = Ok(42);
    println!("expect Ok = {}", value.expect("value honi chahiye thi"));

    // ---------- 3) Manual panic (guard condition) ----------
    // Hum khud panic! macro call kar sakte hai jab kuch bilkul galat ho.
    let age = 25;
    if age > 150 {
        panic!("Age galat hai: {}", age); // ye nahi chalega (age 25 hai)
    }
    println!("Age valid hai: {}", age);

    // ---------- 4) Safe tarika: panic ki jagah default value ----------
    let maybe: Option<i32> = None;
    // unwrap ke bajaye unwrap_or -> panic se bach gaye
    println!("unwrap_or se safe = {}", maybe.unwrap_or(0));

    // ---------- 5) Array index panic ka example (comment) ----------
    let arr = [1, 2, 3];
    // println!("{}", arr[10]); // PANIC: index out of bounds
    println!("Safe access = {:?}", arr.get(10)); // None -> koi crash nahi

    println!("Program safely khatam hua ✅");
}
