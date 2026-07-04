// ============================================================
// TOPIC: Channels (mpsc)  (threads ke beech message bhejna)
// ============================================================
// Run:  rustc channels.rs && ./channels
// ------------------------------------------------------------
// Channel = ek "pipe" jisse ek thread doosri thread ko message bhej sakti hai.
// mpsc = "Multiple Producer, Single Consumer"
//   tx (transmitter) -> bhejne wala
//   rx (receiver)    -> lene wala

use std::sync::mpsc;
use std::thread;

fn main() {
    // ---------- 1) Ek simple message ----------
    // channel() do cheezein deta hai: (tx, rx)
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("Namaste main thread se");
        tx.send(msg).unwrap(); // message bheja
        // NOTE: send ke baad msg ka ownership chala gaya
    });

    // recv() -> message aane tak ruko, phir value do
    let received = rx.recv().unwrap();
    println!("Mila message: {}", received);

    // ---------- 2) Kai messages ek ke baad ek ----------
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec!["ek", "do", "teen", "chaar"];
        for m in messages {
            tx2.send(String::from(m)).unwrap();
        }
        // tx2 yaha drop ho jaata hai -> receiver ko pata chal jaata hai
        // ki ab aur message nahi aayenge, isliye loop apne aap ruk jaata hai.
    });

    // rx ko iterator ki tarah use kar sakte hai
    for received in rx2 {
        println!("Received: {}", received);
    }

    // ---------- 3) Multiple producers (tx clone karke) ----------
    let (tx3, rx3) = mpsc::channel();
    let tx3_clone = tx3.clone(); // dusra bhejne wala

    thread::spawn(move || {
        tx3.send("worker A ka message").unwrap();
    });
    thread::spawn(move || {
        tx3_clone.send("worker B ka message").unwrap();
    });

    // dono messages aayenge (order fixed nahi)
    for received in rx3 {
        println!("Multi-producer: {}", received);
    }
    println!("Channels khatam ✅");
}
