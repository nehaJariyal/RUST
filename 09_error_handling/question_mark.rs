// ============================================================
// TOPIC: ? operator  (error ko aage bhejne ka shortcut)
// ============================================================
// Run:  rustc question_mark.rs && ./question_mark
// ------------------------------------------------------------
// `?` operator ka kaam:
//   * agar Result Ok(v) hai -> v nikaal deta hai aur aage chalta hai
//   * agar Err(e) hai       -> turant us error ko RETURN kar deta hai
// Isse bar-bar match likhne ki zarurat nahi padti. Bahut clean code.

fn main() {
    // ? use karne wala function khud Result return karta hai,
    // isliye main me uska result match/print karte hai.
    match calculate("10", "2") {
        Ok(result) => println!("10 + 2 phir /2... = {}", result),
        Err(e) => println!("Error: {}", e),
    }

    // Yaha "abc" parse fail hoga -> ? turant Err return karega
    match calculate("abc", "2") {
        Ok(result) => println!("Result = {}", result),
        Err(e) => println!("Error: {}", e), // ye chalega
    }

    // Yaha divide by zero
    match calculate("10", "0") {
        Ok(result) => println!("Result = {}", result),
        Err(e) => println!("Error: {}", e), // ye chalega
    }
}

// Do string leta hai, dono parse karta hai, jodta hai, phir 2 se divide.
// Har step Result deta hai -> `?` se error automatically aage chala jaata hai.
fn calculate(a: &str, b: &str) -> Result<i32, String> {
    // parse Result deta hai. Agar Err hua to yahi se return ho jayega.
    let x = parse_number(a)?; // <-- ? ka jaadu
    let y = parse_number(b)?;

    let sum = x + y;
    let result = divide(sum, 2)?; // divide bhi Result deta hai
    Ok(result)
}

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{}' number nahi hai", s))
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Zero se divide nahi"))
    } else {
        Ok(a / b)
    }
}
