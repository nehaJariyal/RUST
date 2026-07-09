// ============================================================
// TOPIC: async / await  (waiting without blocking a thread)
// ============================================================
// Run:  rustc --edition 2024 async_await.rs && ./async_await
//   (NOTE: `async`/`await` requires edition 2018 or newer,
//    so --edition 2024 is required here. Other files don't need
//    this flag. In a cargo project, edition is set in Cargo.toml.)
// ------------------------------------------------------------
// async fn returns a "Future" -> work that has started but is not
// finished yet. `.await` waits for that work's result
// (without blocking the thread).
//
// In REAL projects we use a runtime like "tokio" or "async-std"
// to run futures. Here, without any external crate, we built a
// SMALL custom block_on() for learning (using only std).

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

// ---------- 1) async functions ----------
// With async, the function doesn't return a value directly, it returns a Future.
async fn get_number() -> i32 {
    10
}

async fn double_it(x: i32) -> i32 {
    x * 2
}

// one async fn can .await another async fn
async fn compute() -> i32 {
    let n = get_number().await;   // first get the number
    let d = double_it(n).await;   // then double it
    println!("compute ke andar: n={}, double={}", n, d);
    d + 5
}

fn main() {
    // block_on -> run the Future and wait until the result is ready
    let result = block_on(compute());
    println!("Final result = {}", result);

    // another example
    let num = block_on(get_number());
    println!("get_number() = {}", num);
}

// ============================================================
// The code below is a MINI async runtime (advanced - no need to
// memorize now). Just understand: it "polls" the Future repeatedly
// until it becomes Ready.
// ============================================================

fn block_on<F: Future>(mut future: F) -> F::Output {
    // pin the future to a fixed location (required for async)
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    // create a "noop" waker (does nothing special)
    let waker = noop_waker();
    let mut context = Context::from_waker(&waker);

    // keep polling until Ready
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(value) => return value, // work done -> return value
            Poll::Pending => continue,          // not done yet -> try again
        }
    }
}

// A waker that does nothing (enough for our simple loop)
fn noop_waker() -> Waker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}
