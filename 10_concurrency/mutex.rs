// ============================================================
// TOPIC: Mutex + Arc  (multiple threads safely sharing the same data)
// ============================================================
// Run:  rustc mutex.rs && ./mutex
// ------------------------------------------------------------
// Problem: multiple threads wanting to change the same value can cause a "data race".
// Solution:
//   Mutex<T> -> "Mutual Exclusion" -> only one thread at a time can lock and
//               touch the data.
//   Arc<T>   -> "Atomic Reference Counted" -> safely share the same data
//               across threads (clone to pass around).

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // ---------- Shared counter that 10 threads will increment ----------
    // Arc::new -> made shareable, Mutex::new -> made lock-protected
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // Clone the Arc -> this is a new "reference" to the same data,
        // not a full copy of the data (only the counter increases).
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // Must acquire lock() -> gives a MutexGuard
            // (if another thread holds the lock, this will wait here)
            let mut num = counter.lock().unwrap();
            *num += 1; // now safely increment the value
            // lock is automatically released when we leave this scope
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // 10 threads each added 1 -> final value should be 10
    println!("Final counter = {}", *counter.lock().unwrap());
}
