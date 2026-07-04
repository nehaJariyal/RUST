// ============================================================
// TOPIC: Trait Bounds  (generic par shart ki wo trait implement kare)
// ============================================================
// Run:  rustc trait_bounds.rs && ./trait_bounds
// ------------------------------------------------------------
// Trait bound = generic <T> par condition lagana ki T kaunsa trait
// implement karta ho. Tabhi hum us trait ke methods use kar sakte hai.

use std::fmt::Display;

// ---------- 1) Ek trait banate hai ----------
trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    content: String,
}

struct Tweet {
    user: String,
    text: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{} - {}...", self.title, &self.content[..self.content.len().min(10)])
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.user, self.text)
    }
}

// ---------- 2) Trait bound do tarike se likhte hai ----------

// (a) Short syntax: impl Trait
fn notify(item: &impl Summary) {
    println!("News! {}", item.summarize());
}

// (b) Generic syntax: <T: Trait>  (dono barabar hai)
fn notify_generic<T: Summary>(item: &T) {
    println!("Generic News! {}", item.summarize());
}

// ---------- 3) Ek se zyada trait bound (+) ----------
// T ko Summary bhi implement karna hai aur Display bhi
fn notify_and_print<T: Summary + Display>(item: &T) {
    println!("Print: {}", item);
    println!("Summary: {}", item.summarize());
}

// ---------- 4) where clause (zyada bounds ho to saaf dikhta hai) ----------
fn compare<T>(a: T, b: T) -> bool
where
    T: PartialEq,
{
    a == b
}

fn main() {
    let article = Article {
        title: String::from("Rust Mast Hai"),
        content: String::from("Rust ek systems programming language hai"),
    };
    let tweet = Tweet {
        user: String::from("neha"),
        text: String::from("Aaj traits seekhe!"),
    };

    notify(&article);
    notify_generic(&tweet);

    println!("compare(5, 5) = {}", compare(5, 5));
    println!("compare(\"a\", \"b\") = {}", compare("a", "b"));
}
