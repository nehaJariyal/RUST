// ============================================================
// TOPIC: Tuple Structs & Unit Structs
// ============================================================
// Run:  rustc tuple_structs.rs && ./tuple_structs
// ------------------------------------------------------------
// Tuple struct = a struct without field NAMES, only types.
// Useful when you don't need names but want a distinct type.

// Tuple struct: RGB color (three u8 values)
#[derive(Debug)]
struct Color(u8, u8, u8);

// Tuple struct: 2D point
#[derive(Debug)]
struct Point(f64, f64);

// UNIT struct: no data (used like a marker)
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // ---------- 1) Creating a tuple struct ----------
    let black = Color(0, 0, 0);
    let red = Color(255, 0, 0);

    // Access fields by index (.0, .1, .2)
    println!("Red -> R:{} G:{} B:{}", red.0, red.1, red.2);
    println!("Black = {:?}", black);

    // ---------- 2) Destructuring ----------
    let Point(x, y) = Point(3.5, 7.2);
    println!("Point -> x = {}, y = {}", x, y);

    // ---------- 3) Benefit of type safety ----------
    // Color and Point are both "two/three numbers" but different TYPES,
    // so you can't accidentally use one in place of the other.
    let origin = Point(0.0, 0.0);
    println!("Distance origin se = {:.2}", distance(&origin, &Point(3.0, 4.0)));

    // ---------- 4) Unit struct ----------
    let _u = AlwaysEqual;
    println!("Unit struct ban gaya (koi data nahi)");
}

// Distance between two points (Pythagoras)
fn distance(a: &Point, b: &Point) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}
