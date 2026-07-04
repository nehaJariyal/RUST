// ============================================================
// TOPIC: Methods  (struct ke saath functions jodna - impl)
// ============================================================
// Run:  rustc methods.rs && ./methods
// ------------------------------------------------------------
// Method = struct ke andar ka function. `impl` block me likhte hai.
//   * &self       -> struct ko sirf padhne ke liye borrow
//   * &mut self    -> struct ko badalne ke liye
//   * (no self)    -> associated function (jaise `new`) - :: se call
//   * self         -> ownership le leta hai (kam use hota hai)

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

// impl block me is struct ke saare methods aate hai
impl Rectangle {
    // Associated function (constructor jaisa) -> self nahi leta
    // Isse Rectangle::new(...) se call karte hai
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    // Method: area calculate karta hai (sirf padhta hai -> &self)
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Method jo dusre Rectangle se compare karta hai
    fn is_bigger_than(&self, other: &Rectangle) -> bool {
        self.area() > other.area()
    }

    // Mutable method: struct ko badalta hai (&mut self)
    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    // Associated function se banaya
    let mut rect = Rectangle::new(4.0, 5.0);
    println!("Rectangle = {:?}", rect);

    // Methods ko dot (.) se call karte hai
    println!("Area = {}", rect.area());
    println!("Perimeter = {}", rect.perimeter());

    let small = Rectangle::new(2.0, 2.0);
    println!("Kya rect bada hai small se? {}", rect.is_bigger_than(&small));

    // Mutable method call -> rect badal jayega
    rect.scale(2.0);
    println!("Scale ke baad = {:?}, naya area = {}", rect, rect.area());
}
