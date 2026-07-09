// ============================================================
// TOPIC: Loops  (loop, while, for) - repeating work
// ============================================================
// Run:  rustc loops.rs && ./loops
// ------------------------------------------------------------

fn main() {
    // ---------- 1) loop -> infinite loop (you must break yourself) ----------
    let mut count = 1;
    let result = loop {
        println!("loop chal raha hai, count = {}", count);
        if count == 3 {
            // you can also return a value with break
            break count * 10;
        }
        count += 1; // increment count or you'll get an infinite loop
    };
    println!("loop se mila result = {}", result);

    // ---------- 2) while -> runs as long as the condition is true ----------
    let mut n = 1;
    while n <= 5 {
        println!("while: n = {}", n);
        n += 1;
    }

    // ---------- 3) for -> iterate over a range or collection (most common) ----------
    // 1..5  => 1,2,3,4 (5 NOT included)
    // 1..=5 => 1,2,3,4,5 (5 included)
    for i in 1..=5 {
        println!("for: i = {}", i);
    }

    // iterating over an array with a for loop
    let fruits = ["aam", "seb", "kela"];
    for fruit in fruits {
        println!("Fruit: {}", fruit);
    }

    // iterate with index -> enumerate()
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{} -> {}", index, fruit);
    }

    // ---------- 4) continue and break ----------
    // continue -> skip the current iteration, start the next one
    // break    -> stop the loop entirely
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // even numbers skip
        }
        if i > 7 {
            break;    // stop after 7
        }
        println!("odd aur <=7: {}", i);
    }

    // ---------- 5) LABELED loop (which loop to break in nested loops) ----------
    'bahar: for i in 1..=3 {
        for j in 1..=3 {
            if i + j == 4 {
                println!("i+j==4 mila, bahar wala loop todo");
                break 'bahar; // breaks the outer 'bahar loop, not just the inner one
            }
            println!("i={}, j={}", i, j);
        }
    }
}
