// ============================================================
// TOPIC: Traits  (shared behaviour - like an interface)
// ============================================================
// Run:  rustc traits.rs && ./traits
// ------------------------------------------------------------
// Trait = a set of methods that any type can implement.
// (Similar to "interface" in Java/C#.)
// This lets you add common behaviour to different types.

// Defining a trait
trait Animal {
    // signature only (no body) -> each type must implement it
    fn sound(&self) -> String;

    // DEFAULT method -> used if the type doesn't override it
    fn describe(&self) -> String {
        format!("Ye animal bolta hai: {}", self.sound())
    }
}

struct Dog;
struct Cat;
struct Cow;

// Implement the trait for each struct
impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("Bhau Bhau")
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meaon")
    }
    // Cat overrides the default describe
    fn describe(&self) -> String {
        String::from("Main billi hu aur main special hu 😺")
    }
}

impl Animal for Cow {
    fn sound(&self) -> String {
        String::from("Ambaa")
    }
}

fn main() {
    let d = Dog;
    let c = Cat;
    let cow = Cow;

    // All of them have .sound() and .describe()
    println!("Dog: {}", d.sound());
    println!("{}", d.describe());   // will use default describe

    println!("Cat: {}", c.sound());
    println!("{}", c.describe());   // Cat's own describe

    println!("Cow: {}", cow.describe());

    // ---------- Using a trait as a parameter ----------
    // &impl Animal -> "any type that implements Animal"
    make_sound(&d);
    make_sound(&cow);

    // ---------- Trait object (dyn) -> different types in one Vec ----------
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Cow)];
    for a in &animals {
        println!("dyn -> {}", a.sound());
    }
}

// Accepts any Animal
fn make_sound(animal: &impl Animal) {
    println!("Ye bola: {}", animal.sound());
}
