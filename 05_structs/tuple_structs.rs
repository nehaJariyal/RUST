// ============================================================
// TOPIC: Tuple Structs & Unit Structs
// ============================================================
// Run:  rustc tuple_structs.rs && ./tuple_structs
// ------------------------------------------------------------
// Tuple struct = struct jisme field ke NAAM nahi hote, sirf types.
// Useful jab naam ki zarurat nahi par ek alag type chahiye.

// Tuple struct: RGB color (teen u8 values)
#[derive(Debug)]
struct Color(u8, u8, u8);

// Tuple struct: 2D point
#[derive(Debug)]
struct Point(f64, f64);

// UNIT struct: koi data nahi (marker ki tarah use hota hai)
#[derive(Debug)]
struct AlwaysEqual;

fn main() {
    // ---------- 1) Tuple struct banana ----------
    let black = Color(0, 0, 0);
    let red = Color(255, 0, 0);

    // Fields ko index (.0, .1, .2) se access karte hai
    println!("Red -> R:{} G:{} B:{}", red.0, red.1, red.2);
    println!("Black = {:?}", black);

    // ---------- 2) Destructuring ----------
    let Point(x, y) = Point(3.5, 7.2);
    println!("Point -> x = {}, y = {}", x, y);

    // ---------- 3) Type safety ka fayda ----------
    // Color aur Point dono "do/teen numbers" hai par alag TYPE hai,
    // isliye galti se ek dusre ki jagah use nahi kar sakte.
    let origin = Point(0.0, 0.0);
    println!("Distance origin se = {:.2}", distance(&origin, &Point(3.0, 4.0)));

    // ---------- 4) Unit struct ----------
    let _u = AlwaysEqual;
    println!("Unit struct ban gaya (koi data nahi)");
}

// Do points ke beech ki doori (Pythagoras)
fn distance(a: &Point, b: &Point) -> f64 {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx * dx + dy * dy).sqrt()
}
