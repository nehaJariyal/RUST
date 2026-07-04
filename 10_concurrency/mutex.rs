// ============================================================
// TOPIC: Mutex + Arc  (kai threads ek hi data safely share kare)
// ============================================================
// Run:  rustc mutex.rs && ./mutex
// ------------------------------------------------------------
// Problem: kai threads ek hi value badalna chahe to "data race" ho sakti hai.
// Solution:
//   Mutex<T> -> "Mutual Exclusion" -> ek time par sirf ek thread lock lekar
//               data ko chhu sakta hai.
//   Arc<T>   -> "Atomic Reference Counted" -> ek hi data ko kai threads ke
//               beech safely share karne ke liye (clone karke bhejte hai).

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // ---------- Shared counter jise 10 threads badhayenge ----------
    // Arc::new -> shareable banaya, Mutex::new -> lock-protected banaya
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        // Arc ko clone karte hai -> ye asli data ki nayi "reference" hai,
        // poora data copy nahi hota (bs counter badh jaata hai).
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            // lock() lena zaruri -> ye MutexGuard deta hai
            // (agar koi aur thread ke paas lock hai to yaha wait karega)
            let mut num = counter.lock().unwrap();
            *num += 1; // ab safely value badhao
            // yaha se nikalte hi lock automatically chhoot jaata hai
        });
        handles.push(handle);
    }

    // Saare threads khatam hone ka intezaar
    for handle in handles {
        handle.join().unwrap();
    }

    // 10 threads ne 1-1 badhaya -> final value 10 honi chahiye
    println!("Final counter = {}", *counter.lock().unwrap());
}
