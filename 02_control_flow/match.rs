// ============================================================
// TOPIC: match  (powerful switch jaisa, har case handle karta hai)
// ============================================================
// Run:  rustc match.rs && ./match
// ------------------------------------------------------------
// match ka rule: EXHAUSTIVE hona chahiye matlab saari possibilities
// cover karni padti hai (warna compiler error deta hai).

fn main() {
    // ---------- 1) SIMPLE match on integer ----------
    let day = 3;
    let name = match day {
        1 => "Somwar",
        2 => "Mangalwar",
        3 => "Budhwar",
        4 => "Guruwar",
        5 => "Shukrawar",
        // `_` matlab "baaki sab" (default case)
        _ => "Weekend",
    };
    println!("Day {} = {}", day, name);

    // ---------- 2) Ek arm me multiple values (|) aur range (..=) ----------
    let number = 7;
    match number {
        0 => println!("Zero"),
        1 | 2 | 3 => println!("Chhota number (1-3)"),
        4..=9 => println!("Medium number (4 se 9)"),
        _ => println!("Bada number"),
    }

    // ---------- 3) match VALUE ko bind karna ----------
    // Range match karke actual value pakad sakte hai
    let age = 25;
    match age {
        n @ 0..=12 => println!("Bachcha, umar {}", n),
        n @ 13..=19 => println!("Teenager, umar {}", n),
        n => println!("Adult, umar {}", n),
    }

    // ---------- 4) match with GUARD (extra if condition) ----------
    let pair = (2, -2);
    match pair {
        (x, y) if x + y == 0 => println!("Sum zero hai"),
        (x, _) if x % 2 == 0 => println!("Pehla even hai"),
        _ => println!("Kuch aur"),
    }

    // ---------- 5) match Option (Some / None) ----------
    // Option ek enum hai: value ho sakti hai (Some) ya na ho (None)
    let maybe_value: Option<i32> = Some(10);
    match maybe_value {
        Some(v) => println!("Value mili: {}", v),
        None => println!("Koi value nahi"),
    }
}
