// ============================================================
// TOPIC: Generics  (ek hi code jo alag alag types par chale)
// ============================================================
// Run:  rustc generics.rs && ./generics
// ------------------------------------------------------------
// Generic = type ko <T> se placeholder ki tarah likhna, taaki same
// function/struct kai types ke saath kaam kare. (Code repeat na ho.)

// ---------- 1) Generic FUNCTION ----------
// <T: PartialOrd> matlab T aisa type ho jise compare (>) kar sake
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
// Point kisi bhi type ke coordinates rakh sakta hai
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

// Generic struct par methods
impl<T: std::fmt::Display> Point<T> {
    fn show(&self) {
        println!("Point -> x: {}, y: {}", self.x, self.y);
    }
}

// ---------- 3) Do alag generic types ----------
#[derive(Debug)]
struct Pair<A, B> {
    first: A,
    second: B,
}

fn main() {
    // Generic function -> integer par
    let numbers = vec![3, 7, 2, 9, 4];
    println!("Sabse bada number = {}", largest(&numbers));

    // Wahi function -> char par (bina code badle!)
    let chars = vec!['a', 'z', 'm', 'k'];
    println!("Sabse bada char = {}", largest(&chars));

    // float par
    let floats = vec![1.5, 3.9, 2.2];
    println!("Sabse bada float = {}", largest(&floats));

    // Generic struct -> integer
    let p1 = Point { x: 5, y: 10 };
    p1.show();

    // Generic struct -> float
    let p2 = Point { x: 1.1, y: 2.2 };
    p2.show();

    // Do alag types ek saath
    let mix = Pair { first: "umar", second: 21 };
    println!("Pair = {:?}", mix);
}
