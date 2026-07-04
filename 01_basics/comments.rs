// ============================================================
// TOPIC: Comments in Rust  (comment likhne ke tarike)
// ============================================================
// Run:  rustc comments.rs && ./comments
// ------------------------------------------------------------

// 1) LINE COMMENT
// Ye // se start hoti hai aur line ke end tak chalti hai.
// Compiler ise ignore kar deta hai.

/*
 2) BLOCK COMMENT
 Ye /* se start aur */ par khatam hoti hai.
 Ek se zyada lines cover kar sakti hai.
 Block comments ke andar block comment /* nested */ bhi chalta hai (Rust me allowed).
*/

/// 3) DOC COMMENT (outer) -> teen slash `///`
/// Ye documentation ke liye hoti hai (function/struct ke UPAR likhte hai).
/// `cargo doc` chalane par ye HTML docs me dikhti hai.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    //! Note: `//!` inner doc comment hoti hai jo apne parent item (module/file)
    //! ko document karti hai. Ye kisi item ke ANDAR pehli line par aati hai.

    let x = 5 + /* beech me bhi comment daal sakte hai */ 5;
    println!("x = {}", x); // isko 10 aana chahiye

    // Comments code samajhne aur explain karne ke liye bahut useful hote hai
    let sum = add(4, 6); // add() function call kiya
    println!("add(4, 6) = {}", sum);

    /* Neeche wali line comment ki hui hai isliye run nahi hogi:
    println!("Ye print nahi hoga");
    */

    println!("Comments seekh liye! ✅");
}
