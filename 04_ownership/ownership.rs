// ============================================================
// TOPIC: Ownership  (Rust ka SABSE important concept)
// ============================================================
// Run:  rustc ownership.rs && ./ownership
// ------------------------------------------------------------
// Ownership ke 3 rules:
//   1) Har value ka ek OWNER (maalik) hota hai.
//   2) Ek time par SIRF EK owner hota hai.
//   3) Jab owner scope se bahar jaata hai -> value memory se hat jati hai (drop).
// Isi wajah se Rust ko garbage collector ki zarurat nahi padti.

fn main() {
    // ---------- 1) SCOPE ka concept ----------
    {
        let s = String::from("scope ke andar");
        println!("{}", s);
    } // <- yaha `s` ka scope khatam, memory automatically free ho gayi
    // println!("{}", s); // <-- ERROR: s ab exist nahi karta

    // ---------- 2) MOVE (heap data ka owner badalna) ----------
    // String heap par banti hai. Jab dusre variable ko de do to
    // ownership "move" ho jaati hai, purana variable invalid ho jaata hai.
    let s1 = String::from("hello");
    let s2 = s1; // ownership s1 -> s2 me MOVE ho gayi
    // println!("{}", s1); // <-- ERROR: s1 ab valid nahi (moved)
    println!("s2 = {}", s2); // s2 valid hai

    // ---------- 3) CLONE (asli copy banana) ----------
    // Agar dono chahiye to .clone() se heap data ki poori copy banao.
    let a = String::from("duplicate");
    let b = a.clone(); // ab a aur b dono alag alag apni value ke owner
    println!("a = {}, b = {}", a, b);

    // ---------- 4) COPY types (stack par, move nahi hota) ----------
    // i32, f64, bool, char, aur inke tuples "Copy" hote hai.
    // Inme move ke bajaye simple copy hoti hai -> dono valid rehte hai.
    let x = 5;
    let y = x; // copy hui, move nahi
    println!("x = {}, y = {} (dono valid)", x, y);

    // ---------- 5) Function ko value dena = ownership move ----------
    let text = String::from("main function ko jaunga");
    take_ownership(text); // ownership function ke andar move ho gayi
    // println!("{}", text); // <-- ERROR: text moved ho chuka

    // ---------- 6) Ownership wapas lena (return se) ----------
    let given = String::from("data");
    let returned = give_back(given); // andar gaya, wapas mila
    println!("wapas mila: {}", returned);
}

// Ye function String ka owner ban jaata hai, function khatam hote hi drop
fn take_ownership(s: String) {
    println!("take_ownership ke andar: {}", s);
} // yaha s drop ho jaata hai

// Ownership andar leta hai aur return karke wapas de deta hai
fn give_back(s: String) -> String {
    s // ownership wapas caller ko chali jaati hai
}
