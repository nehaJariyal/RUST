// ============================================================
// TOPIC: Pattern Matching  (breaking values apart to look inside)
// ============================================================
// Run:  rustc pattern_matching.rs && ./pattern_matching
// ------------------------------------------------------------
// Pattern matching is not limited to match. It's also used in let,
// if let, while let, for, and tuple/struct destructuring.

// Create an enum so we can demonstrate pattern matching on it
#[derive(Debug)]
enum Shape {
    Circle(f64),           // one value: radius
    Rectangle(f64, f64),   // two values: width, height
    Point,                 // no values
}

fn main() {
    // ---------- 1) DESTRUCTURING with let (breaking apart a tuple) ----------
    let point = (3, 5);
    let (x, y) = point; // tuple split into x and y
    println!("x = {}, y = {}", x, y);

    // ---------- 2) if let -> when you only care about ONE case ----------
    // A short shortcut instead of writing a full match.
    let some_number: Option<i32> = Some(42);
    if let Some(n) = some_number {
        println!("if let se value mili: {}", n);
    } else {
        println!("None tha");
    }

    // ---------- 3) while let -> loop as long as the pattern matches ----------
    let mut stack = vec![1, 2, 3];
    // pop() returns Option: Some(value) or None (when empty)
    while let Some(top) = stack.pop() {
        println!("Stack se nikala: {}", top);
    }

    // ---------- 4) Extracting values inside an enum ----------
    let shapes = [Shape::Circle(2.0), Shape::Rectangle(3.0, 4.0), Shape::Point];
    for shape in shapes {
        match shape {
            Shape::Circle(r) => println!("Circle -> area = {:.2}", 3.14 * r * r),
            Shape::Rectangle(w, h) => println!("Rectangle -> area = {}", w * h),
            Shape::Point => println!("Point ka koi area nahi"),
        }
    }

    // ---------- 5) Ignoring parts with `_` and `..` ----------
    let numbers = (1, 2, 3, 4, 5);
    let (first, .., last) = numbers; // ignore the middle, keep only first and last
    println!("Pehla = {}, aakhri = {}", first, last);
}
