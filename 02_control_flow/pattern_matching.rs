// ============================================================
// TOPIC: Pattern Matching  (values ko todkar unke andar jhaankna)
// ============================================================
// Run:  rustc pattern_matching.rs && ./pattern_matching
// ------------------------------------------------------------
// Pattern matching sirf match tak simit nahi hai. Ye let, if let,
// while let, for aur tuple/struct destructuring me bhi use hota hai.

// Ek enum banate hai taaki uspar pattern match dikha sake
#[derive(Debug)]
enum Shape {
    Circle(f64),           // ek value: radius
    Rectangle(f64, f64),   // do value: width, height
    Point,                 // koi value nahi
}

fn main() {
    // ---------- 1) let se DESTRUCTURING (tuple todna) ----------
    let point = (3, 5);
    let (x, y) = point; // tuple ko x aur y me tod diya
    println!("x = {}, y = {}", x, y);

    // ---------- 2) if let -> jab sirf EK case me interest ho ----------
    // Poora match likhne ke bajaye chhota shortcut.
    let some_number: Option<i32> = Some(42);
    if let Some(n) = some_number {
        println!("if let se value mili: {}", n);
    } else {
        println!("None tha");
    }

    // ---------- 3) while let -> jab tak pattern match ho tab tak loop ----------
    let mut stack = vec![1, 2, 3];
    // pop() Option deta hai: Some(value) ya None (jab khali ho jaye)
    while let Some(top) = stack.pop() {
        println!("Stack se nikala: {}", top);
    }

    // ---------- 4) enum ke andar ki values nikalna ----------
    let shapes = [Shape::Circle(2.0), Shape::Rectangle(3.0, 4.0), Shape::Point];
    for shape in shapes {
        match shape {
            Shape::Circle(r) => println!("Circle -> area = {:.2}", 3.14 * r * r),
            Shape::Rectangle(w, h) => println!("Rectangle -> area = {}", w * h),
            Shape::Point => println!("Point ka koi area nahi"),
        }
    }

    // ---------- 5) `_` aur `..` se cheezein ignore karna ----------
    let numbers = (1, 2, 3, 4, 5);
    let (first, .., last) = numbers; // beech ke ignore, sirf pehla aur aakhri
    println!("Pehla = {}, aakhri = {}", first, last);
}
