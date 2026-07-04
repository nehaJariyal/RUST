// ============================================================
// TOPIC: Loops  (loop, while, for) - kaam repeat karna
// ============================================================
// Run:  rustc loops.rs && ./loops
// ------------------------------------------------------------

fn main() {
    // ---------- 1) loop -> infinite loop (khud break karna padta hai) ----------
    let mut count = 1;
    let result = loop {
        println!("loop chal raha hai, count = {}", count);
        if count == 3 {
            // break ke saath value bhi return kar sakte hai
            break count * 10;
        }
        count += 1; // count ko badhao warna infinite loop ho jayega
    };
    println!("loop se mila result = {}", result);

    // ---------- 2) while -> jab tak condition true hai tab tak chalega ----------
    let mut n = 1;
    while n <= 5 {
        println!("while: n = {}", n);
        n += 1;
    }

    // ---------- 3) for -> range ya collection par ghumna (sabse common) ----------
    // 1..5  => 1,2,3,4 (5 shamil NAHI)
    // 1..=5 => 1,2,3,4,5 (5 bhi shamil)
    for i in 1..=5 {
        println!("for: i = {}", i);
    }

    // for loop se array par iterate karna
    let fruits = ["aam", "seb", "kela"];
    for fruit in fruits {
        println!("Fruit: {}", fruit);
    }

    // index ke saath ghumna -> enumerate()
    for (index, fruit) in fruits.iter().enumerate() {
        println!("{} -> {}", index, fruit);
    }

    // ---------- 4) continue aur break ----------
    // continue -> current chakkar chhodo, agla shuru karo
    // break    -> loop poora band karo
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // even numbers skip
        }
        if i > 7 {
            break;    // 7 ke baad ruk jao
        }
        println!("odd aur <=7: {}", i);
    }

    // ---------- 5) LABELED loop (nested loop me kaunsa break karna hai) ----------
    'bahar: for i in 1..=3 {
        for j in 1..=3 {
            if i + j == 4 {
                println!("i+j==4 mila, bahar wala loop todo");
                break 'bahar; // sirf andar wala nahi, 'bahar wala tootega
            }
            println!("i={}, j={}", i, j);
        }
    }
}
