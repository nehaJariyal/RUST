// ============================================================
// TOPIC: if / else  (condition ke hisaab se decision lena)
// ============================================================
// Run:  rustc if_else.rs && ./if_else
// ------------------------------------------------------------

fn main() {
    let number = 15;

    // 1) SIMPLE if / else
    // Condition me () lagana zaruri NAHI hai, par {} block ZARURI hai.
    if number % 2 == 0 {
        println!("{} even hai", number);
    } else {
        println!("{} odd hai", number);
    }

    // 2) else if -> ek se zyada condition check karne ke liye
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

    // 3) if ko EXPRESSION ki tarah use karna
    // Rust me if value return kar sakta hai, isliye let ke saath likh sakte hai.
    // Dhyaan: dono branch ka type same hona chahiye.
    let is_big = if number > 10 { "bada" } else { "chhota" };
    println!("Number {} hai", is_big);

    // 4) Nested if (if ke andar if)
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
