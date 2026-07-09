// ============================================================
// TOPIC: Channels (mpsc)  (sending messages between threads)
// ============================================================
// Run:  rustc channels.rs && ./channels
// ------------------------------------------------------------
// Channel = a "pipe" through which one thread can send a message to another.
// mpsc = "Multiple Producer, Single Consumer"
//   tx (transmitter) -> sender
//   rx (receiver)    -> receiver

use std::sync::mpsc;
use std::thread;

fn main() {
    // ---------- 1) A simple message ----------
    // channel() gives two things: (tx, rx)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Namaste main thread se");
        tx.send(msg).unwrap(); // sent the message
        // NOTE: after send, msg's ownership moved away
    });

    // recv() -> wait for a message, then return the value
    let received = rx.recv().unwrap();
    println!("Mila message: {}", received);

    // ---------- 2) Multiple messages one after another ----------
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["ek", "do", "teen", "chaar"];
        for m in messages {
            tx2.send(String::from(m)).unwrap();
        }
        // tx2 is dropped here -> receiver knows
        // no more messages are coming, so the loop stops on its own.
    });

    // rx can be used like an iterator
    for received in rx2 {
        println!("Received: {}", received);
    }

    // ---------- 3) Multiple producers (by cloning tx) ----------
    let (tx3, rx3) = mpsc::channel();
    let tx3_clone = tx3.clone(); // second sender

    thread::spawn(move || {
        tx3.send("worker A ka message").unwrap();
    });
    thread::spawn(move || {
        tx3_clone.send("worker B ka message").unwrap();
    });

    // both messages will arrive (order not fixed)
    for received in rx3 {
        println!("Multi-producer: {}", received);
    }
    println!("Channels khatam ✅");
}
