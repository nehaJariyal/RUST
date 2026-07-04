// ============================================================
// TOPIC: Borrowing  (ownership diye bina value udhaar dena)
// ============================================================
// Run:  rustc borrowing.rs && ./borrowing
// ------------------------------------------------------------
// Har baar ownership move karna painful hai. Isliye Rust me hum
// value ko "borrow" (udhaar) kar sakte hai `&` (reference) se.
// Borrowing ke rules:
//   * Ya to KAI immutable borrow (&)  ...ek saath allowed
//   * Ya SIRF EK mutable borrow (&mut) ...tab koi aur borrow nahi
//   (ye rule "data race" ko compile time par hi rok deta hai)

fn main() {
    // ---------- 1) IMMUTABLE borrow (& ) ----------
    // Function ko reference bhejte hai -> ownership nahi jaati.
    let s = String::from("hello");
    let len = calculate_length(&s); // &s = s ka reference udhaar diya
    println!("'{}' ki length = {} (s abhi bhi mera hai)", s, len);

    // ---------- 2) Ek saath kai immutable borrow ----------
    let data = String::from("shared");
    let r1 = &data;
    let r2 = &data; // dono theek hai kyunki dono sirf padh rahe hai
    println!("r1 = {}, r2 = {}", r1, r2);

    // ---------- 3) MUTABLE borrow (&mut) ----------
    // Isse function value ko badal bhi sakta hai.
    let mut word = String::from("Hi");
    change(&mut word); // mutable reference diya
    println!("change ke baad word = {}", word);

    // ---------- 4) Ek time par sirf EK mutable borrow ----------
    let mut num = 10;
    {
        let m = &mut num; // ek mutable borrow
        *m += 5;          // * se andar ki value badli
        println!("m = {}", m);
    } // m ka scope khatam -> ab dubara borrow kar sakte hai
    let m2 = &mut num;
    *m2 += 100;
    println!("num final = {}", m2);

    // NOTE: neeche wala combination ERROR deta hai -
    // ek mutable aur ek immutable borrow ek saath allowed nahi:
    //   let r = &num;
    //   let w = &mut num;   // ERROR
    //   println!("{} {}", r, w);
}

// & se borrow: hum sirf padh sakte hai, badal nahi sakte
fn calculate_length(s: &String) -> usize {
    s.len()
} // s yaha drop nahi hota kyunki iska owner koi aur hai

// &mut se borrow: hum value ko badal sakte hai
fn change(s: &mut String) {
    s.push_str(" Rust"); // asli string me add ho jayega
}
