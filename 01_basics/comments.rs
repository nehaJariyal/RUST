// ============================================================
// TOPIC: Comments in Rust  (ways to write comments)
// ============================================================
// Run:  rustc comments.rs && ./comments
// ------------------------------------------------------------

// 1) LINE COMMENT
// These start with // and run until the end of the line.
// The compiler ignores them.

/*
 2) BLOCK COMMENT
 These start with /* and end with */.
 They can cover more than one line.
 Block comments can be nested inside block comments /* like this */ (allowed in Rust).
*/

/// 3) DOC COMMENT (outer) -> three slashes `///`
/// These are for documentation (written ABOVE a function/struct).
/// When you run `cargo doc`, they appear in the HTML docs.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    //! Note: `//!` is an inner doc comment that documents its parent item (module/file).
    //! It appears as the first line inside an item.

    let x = 5 + /* you can also put a comment in the middle */ 5;
    println!("x = {}", x); // this should be 10

    // Comments are very useful for understanding and explaining code
    let sum = add(4, 6); // called the add() function
    println!("add(4, 6) = {}", sum);

    /* The line below is commented out so it won't run:
    println!("Ye print nahi hoga");
    */

    println!("Comments seekh liye! ✅");
}
