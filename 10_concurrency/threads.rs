// ============================================================
// TOPIC: Threads  (multiple tasks at once - parallel execution)
// ============================================================
// Run:  rustc threads.rs && ./threads
// ------------------------------------------------------------
// Thread = a separate "path of execution" inside a program. Multiple threads
// work at the same time (in parallel). Created with std::thread.

use std::thread;
use std::time::Duration;

fn main() {
    // ---------- 1) Creating a new thread (spawn) ----------
    // spawn starts a new thread that runs the closure.
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("  [naya thread] gin raha: {}", i);
            thread::sleep(Duration::from_millis(10)); // pause briefly
        }
    });

    // Main thread keeps doing its own work alongside
    for i in 1..=3 {
        println!("[main thread] kaam: {}", i);
        thread::sleep(Duration::from_millis(10));
    }

    // ---------- 2) join() -> wait for the thread to finish ----------
    // Without join, main may finish first and the new
    // thread may not complete its work.
    handle.join().unwrap();
    println!("Naya thread khatam ho gaya");

    // ---------- 3) move closure -> give data to the thread ----------
    // `move` keyword moves variable ownership into the thread
    let numbers = vec![1, 2, 3, 4, 5];
    let handle2 = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("Thread ke andar sum = {}", sum);
        sum // a thread can also return a value
    });
    // join() also gives back the return value
    let result = handle2.join().unwrap();
    println!("Thread se mila result = {}", result);

    // ---------- 4) Multiple threads at once ----------
    let mut handles = vec![];
    for id in 1..=3 {
        let h = thread::spawn(move || {
            println!("Worker #{} kaam kar raha", id);
        });
        handles.push(h);
    }
    // wait for all of them
    for h in handles {
        h.join().unwrap();
    }
    println!("Saare workers khatam ✅");
}
