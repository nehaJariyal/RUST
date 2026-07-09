// ============================================================
// TOPIC: match  (powerful switch-like, handles every case)
// ============================================================
// Run:  rustc match.rs && ./match
// ------------------------------------------------------------
// match rule: must be EXHAUSTIVE, meaning all possibilities
// must be covered (otherwise the compiler gives an error).

fn main() {
    // ---------- 1) SIMPLE match on integer ----------
    let day = 3;
    let name = match day {
        1 => "Somwar",
        2 => "Mangalwar",
        3 => "Budhwar",
        4 => "Guruwar",
        5 => "Shukrawar",
        // `_` means "everything else" (default case)
        _ => "Weekend",
    };
    println!("Day {} = {}", day, name);

    // ---------- 2) Multiple values in one arm (|) and range (..=) ----------
    let number = 7;
    match number {
        0 => println!("Zero"),
        1 | 2 | 3 => println!("Chhota number (1-3)"),
        4..=9 => println!("Medium number (4 se 9)"),
        _ => println!("Bada number"),
    }

    // ---------- 3) Binding the matched VALUE ----------
    // Match a range and capture the actual value
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

    // ---------- 5) match on Option (Some / None) ----------
    // Option is an enum: value may exist (Some) or not (None)
    let maybe_value: Option<i32> = Some(10);
    match maybe_value {
        Some(v) => println!("Value mili: {}", v),
        None => println!("Koi value nahi"),
    }
}
