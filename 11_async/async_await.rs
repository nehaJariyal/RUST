// ============================================================
// TOPIC: async / await  (bina thread block kiye "wait" karna)
// ============================================================
// Run:  rustc --edition 2024 async_await.rs && ./async_await
//   (NOTE: `async`/`await` ke liye edition 2018 ya usse naya chahiye,
//    isliye yaha --edition 2024 lagana zaruri hai. Baaki files me ye
//    flag ki zarurat nahi. cargo project me Cargo.toml se edition set hoti hai.)
// ------------------------------------------------------------
// async fn ek "Future" return karti hai -> ek aisa kaam jo abhi shuru
// hua hai par abhi poora nahi hua. `.await` us kaam ke result ka intezaar
// karta hai (par thread ko block kiye bina).
//
// NORMAL projects me hum "tokio" ya "async-std" jaisa runtime use karte hai
// jo futures ko chalata hai. Yaha bina kisi external crate ke samajhne ke
// liye humne ek CHHOTA sa apna block_on() banaya hai (sirf std se).

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// ---------- 1) async functions ----------
// async lagate hi ye function seedhi value nahi, ek Future return karti hai.
async fn get_number() -> i32 {
    10
}

async fn double_it(x: i32) -> i32 {
    x * 2
}

// ek async fn doosri async fn ko .await kar sakti hai
async fn compute() -> i32 {
    let n = get_number().await;   // pehle number lo
    let d = double_it(n).await;   // phir usse double karo
    println!("compute ke andar: n={}, double={}", n, d);
    d + 5
}

fn main() {
    // block_on -> Future ko chalao aur result milne tak ruko
    let result = block_on(compute());
    println!("Final result = {}", result);

    // ek aur example
    let num = block_on(get_number());
    println!("get_number() = {}", num);
}

// ============================================================
// Neeche ka code ek MINI async runtime hai (advanced - abhi ratne ki
// zarurat nahi). Sirf itna samjho: ye Future ko baar-baar "poll" karta
// hai jab tak wo Ready na ho jaye.
// ============================================================

fn block_on<F: Future>(mut future: F) -> F::Output {
    // future ko ek fixed jagah par pin karte hai (async ke liye zaruri)
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // ek "khaali" waker banate hai (kuch special nahi karta)
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);

    // jab tak Ready na ho, poll karte raho
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value, // kaam poora -> value do
            Poll::Pending => continue,          // abhi baaki -> dubara try
        }
    }
}

// Ek waker jo kuch nahi karta (humare simple loop ke liye kaafi hai)
fn noop_waker() -> Waker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}
