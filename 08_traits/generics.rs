// ============================================================
// TOPIC: Generics  (one piece of code that works on different types)
// ============================================================
// Run:  rustc generics.rs && ./generics
// ------------------------------------------------------------
// Generic = writing type as a <T> placeholder so the same
// function/struct works with many types. (Avoids repeating code.)

// ---------- 1) Generic FUNCTION ----------
// <T: PartialOrd> means T must be a type that can be compared (>)
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item;
        }
    }
    max
}

// ---------- 2) Generic STRUCT ----------
// Point can hold coordinates of any type
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Methods on a generic struct
impl<T: std::fmt::Display> Point<T> {
    fn show(&self) {
        println!("Point -> x: {}, y: {}", self.x, self.y);
    }
}

// ---------- 3) Two different generic types ----------
#[derive(Debug)]
struct Pair<A, B> {
    first: A,
    second: B,
}

fn main() {
    // Generic function -> on integers
    let numbers = vec![3, 7, 2, 9, 4];
    println!("Sabse bada number = {}", largest(&numbers));

    // Same function -> on char (without changing the code!)
    let chars = vec!['a', 'z', 'm', 'k'];
    println!("Sabse bada char = {}", largest(&chars));

    // on float
    let floats = vec![1.5, 3.9, 2.2];
    println!("Sabse bada float = {}", largest(&floats));

    // Generic struct -> integer
    let p1 = Point { x: 5, y: 10 };
    p1.show();

    // Generic struct -> float
    let p2 = Point { x: 1.1, y: 2.2 };
    p2.show();

    // Two different types together
    let mix = Pair { first: "umar", second: 21 };
    println!("Pair = {:?}", mix);
}
