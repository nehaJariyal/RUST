// ============================================================
// TOPIC: panic!  (stopping the program immediately - unrecoverable error)
// ============================================================
// Run:  rustc panic.rs && ./panic
// ------------------------------------------------------------
// Rust has 2 kinds of errors:
//   1) Recoverable   -> Result<T, E> (can be handled)
//   2) Unrecoverable -> panic! (program crashes)
// Use panic when it's not safe to continue.

fn main() {
    // ---------- 1) unwrap -> value on Some/Ok, otherwise PANIC ----------
    let good: Option<i32> = Some(10);
    println!("unwrap Some = {}", good.unwrap()); // will get 10

    // NOTE: the line below PANICS (so it's commented out):
    // let bad: Option<i32> = None;
    // bad.unwrap(); // panic: called `Option::unwrap()` on a `None`

    // ---------- 2) expect -> like unwrap but with a custom message ----------
    let value: Result<i32, String> = Ok(42);
    println!("expect Ok = {}", value.expect("value honi chahiye thi"));

    // ---------- 3) Manual panic (guard condition) ----------
    // We can call the panic! macro ourselves when something is seriously wrong.
    let age = 25;
    if age > 150 {
        panic!("Age galat hai: {}", age); // won't run (age is 25)
    }
    println!("Age valid hai: {}", age);

    // ---------- 4) Safe approach: default value instead of panic ----------
    let maybe: Option<i32> = None;
    // unwrap_or instead of unwrap -> avoided panic
    println!("unwrap_or se safe = {}", maybe.unwrap_or(0));

    // ---------- 5) Array index panic example (comment) ----------
    let arr = [1, 2, 3];
    // println!("{}", arr[10]); // PANIC: index out of bounds
    println!("Safe access = {:?}", arr.get(10)); // None -> no crash

    println!("Program safely khatam hua ✅");
}
