// ============================================================
// TOPIC: Methods  (attaching functions to a struct - impl)
// ============================================================
// Run:  rustc methods.rs && ./methods
// ------------------------------------------------------------
// Method = a function inside a struct. Written in an `impl` block.
//   * &self       -> borrow the struct for reading only
//   * &mut self    -> borrow the struct for mutation
//   * (no self)    -> associated function (like `new`) - called with ::
//   * self         -> takes ownership (rarely used)

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// All methods for this struct go in the impl block
impl Rectangle {
    // Associated function (like a constructor) -> does not take self
    // Called as Rectangle::new(...)
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Method: calculates area (only reads -> &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Method that compares with another Rectangle
    fn is_bigger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // Mutable method: changes the struct (&mut self)
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // Created with associated function
    let mut rect = Rectangle::new(4.0, 5.0);
    println!("Rectangle = {:?}", rect);

    // Methods are called with dot (.)
    println!("Area = {}", rect.area());
    println!("Perimeter = {}", rect.perimeter());

    let small = Rectangle::new(2.0, 2.0);
    println!("Kya rect bada hai small se? {}", rect.is_bigger_than(&small));

    // Mutable method call -> rect will change
    rect.scale(2.0);
    println!("Scale ke baad = {:?}, naya area = {}", rect, rect.area());
}
