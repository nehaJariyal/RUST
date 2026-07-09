// ============================================================
// TOPIC: Trait Bounds  (requiring a generic to implement a trait)
// ============================================================
// Run:  rustc trait_bounds.rs && ./trait_bounds
// ------------------------------------------------------------
// Trait bound = putting a condition on generic <T> that T must
// implement a certain trait. Only then can we use that trait's methods.

use std::fmt::Display;

// ---------- 1) Create a trait ----------
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

// ---------- 2) Two ways to write trait bounds ----------

// (a) Short syntax: impl Trait
fn notify(item: &impl Summary) {
    println!("News! {}", item.summarize());
}

// (b) Generic syntax: <T: Trait>  (both are equivalent)
fn notify_generic<T: Summary>(item: &T) {
    println!("Generic News! {}", item.summarize());
}

// ---------- 3) Multiple trait bounds (+) ----------
// T must implement both Summary and Display
fn notify_and_print<T: Summary + Display>(item: &T) {
    println!("Print: {}", item);
    println!("Summary: {}", item.summarize());
}

// ---------- 4) where clause (cleaner when there are many bounds) ----------
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
