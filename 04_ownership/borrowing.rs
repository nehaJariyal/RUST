// ============================================================
// TOPIC: Borrowing  (lending a value without giving ownership)
// ============================================================
// Run:  rustc borrowing.rs && ./borrowing
// ------------------------------------------------------------
// Moving ownership every time is painful. So in Rust we can
// "borrow" a value using `&` (reference).
// Borrowing rules:
//   * Either MANY immutable borrows (&)  ...allowed at the same time
//   * Or ONLY ONE mutable borrow (&mut) ...no other borrows at that time
//   (this rule stops "data races" at compile time)

fn main() {
    // ---------- 1) IMMUTABLE borrow (& ) ----------
    // Send a reference to the function -> ownership does not move.
    let s = String::from("hello");
    let len = calculate_length(&s); // &s = borrowed a reference to s
    println!("'{}' ki length = {} (s abhi bhi mera hai)", s, len);

    // ---------- 2) Many immutable borrows at the same time ----------
    let data = String::from("shared");
    let r1 = &data;
    let r2 = &data; // both are fine because both are only reading
    println!("r1 = {}, r2 = {}", r1, r2);

    // ---------- 3) MUTABLE borrow (&mut) ----------
    // This lets the function also change the value.
    let mut word = String::from("Hi");
    change(&mut word); // gave a mutable reference
    println!("change ke baad word = {}", word);

    // ---------- 4) Only ONE mutable borrow at a time ----------
    let mut num = 10;
    {
        let m = &mut num; // one mutable borrow
        *m += 5;          // changed the inner value with *
        println!("m = {}", m);
    } // m's scope ends -> can borrow again now
    let m2 = &mut num;
    *m2 += 100;
    println!("num final = {}", m2);

    // NOTE: the combination below gives an ERROR -
    // one mutable and one immutable borrow at the same time is not allowed:
    //   let r = &num;
    //   let w = &mut num;   // ERROR
    //   println!("{} {}", r, w);
}

// & borrow: we can only read, not change
fn calculate_length(s: &String) -> usize {
    s.len()
} // s is not dropped here because its owner is someone else

// &mut borrow: we can change the value
fn change(s: &mut String) {
    s.push_str(" Rust"); // will be added to the original string
}
