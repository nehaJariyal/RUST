// ============================================================
// TOPIC: Threads  (ek saath kai kaam - parallel execution)
// ============================================================
// Run:  rustc threads.rs && ./threads
// ------------------------------------------------------------
// Thread = program ke andar ek alag "chalne wala rasta". Kai threads
// ek saath (parallel) kaam karte hai. std::thread se banate hai.

use std::thread;
use std::time::Duration;

fn main() {
    // ---------- 1) Naya thread banana (spawn) ----------
    // spawn ek nayi thread shuru karta hai jo closure ko chalati hai.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  [naya thread] gin raha: {}", i);
            thread::sleep(Duration::from_millis(10)); // thoda ruko
        }
    });

    // Main thread apna kaam karta rahega saath me
    for i in 1..=3 {
        println!("[main thread] kaam: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // ---------- 2) join() -> thread ke khatam hone ka intezaar ----------
    // Agar join na kare to main pehle khatam ho sakta hai aur naya
    // thread apna kaam poora nahi kar payega.
    handle.join().unwrap();
    println!("Naya thread khatam ho gaya");

    // ---------- 3) move closure -> data thread ko de dena ----------
    // `move` keyword se variable ki ownership thread ke andar chali jaati hai
    let numbers = vec![1, 2, 3, 4, 5];
    let handle2 = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("Thread ke andar sum = {}", sum);
        sum // thread value bhi return kar sakti hai
    });
    // join() se return value bhi milti hai
    let result = handle2.join().unwrap();
    println!("Thread se mila result = {}", result);

    // ---------- 4) Kai threads ek saath ----------
    let mut handles = vec![];
    for id in 1..=3 {
        let h = thread::spawn(move || {
            println!("Worker #{} kaam kar raha", id);
        });
        handles.push(h);
    }
    // sabka intezaar
    for h in handles {
        h.join().unwrap();
    }
    println!("Saare workers khatam ✅");
}
