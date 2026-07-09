// ============================================================
// TOPIC: if / else  (making decisions based on conditions)
// ============================================================
// Run:  rustc if_else.rs && ./if_else
// ------------------------------------------------------------

fn main() {
    let number = 15;

    // 1) SIMPLE if / else
    // Parentheses around the condition are NOT required, but {} block IS required.
    if number % 2 == 0 {
        println!("{} even hai", number);
    } else {
        println!("{} odd hai", number);
    }

    // 2) else if -> for checking more than one condition
    let marks = 82;
    if marks >= 90 {
        println!("Grade: A");
    } else if marks >= 75 {
        println!("Grade: B");
    } else if marks >= 50 {
        println!("Grade: C");
    } else {
        println!("Fail");
    }

    // 3) Using if as an EXPRESSION
    // In Rust, if can return a value, so you can use it with let.
    // Note: both branches must have the same type.
    let is_big = if number > 10 { "bada" } else { "chhota" };
    println!("Number {} hai", is_big);

    // 4) Nested if (if inside if)
    let age = 20;
    let has_id = true;
    if age >= 18 {
        if has_id {
            println!("Entry allowed ✅");
        } else {
            println!("ID nahi hai, entry nahi ❌");
        }
    } else {
        println!("Umar kam hai");
    }

    // 5) Logical operators: && (AND), || (OR), ! (NOT)
    let temp = 30;
    if temp > 25 && temp < 40 {
        println!("Mausam garam par theek hai");
    }
}
