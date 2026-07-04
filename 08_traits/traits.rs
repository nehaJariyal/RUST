// ============================================================
// TOPIC: Traits  (shared behaviour - interface jaisa)
// ============================================================
// Run:  rustc traits.rs && ./traits
// ------------------------------------------------------------
// Trait = ek set of methods jo koi bhi type implement kar sakta hai.
// (Java/C# ke "interface" jaisa concept.)
// Isse alag alag types me common behaviour add karte hai.

// Trait define karna
trait Animal {
    // sirf signature (koi body nahi) -> har type ko khud likhna padega
    fn sound(&self) -> String;

    // DEFAULT method -> agar type override na kare to yahi use hoga
    fn describe(&self) -> String {
        format!("Ye animal bolta hai: {}", self.sound())
    }
}

struct Dog;
struct Cat;
struct Cow;

// Har struct ke liye trait implement karte hai
impl Animal for Dog {
    fn sound(&self) -> String {
        String::from("Bhau Bhau")
    }
}

impl Animal for Cat {
    fn sound(&self) -> String {
        String::from("Meaon")
    }
    // Cat default describe ko override kar rahi hai
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

    // Sabhi ke paas .sound() aur .describe() hai
    println!("Dog: {}", d.sound());
    println!("{}", d.describe());   // default describe use hoga

    println!("Cat: {}", c.sound());
    println!("{}", c.describe());   // Cat ka apna describe

    println!("Cow: {}", cow.describe());

    // ---------- Trait ko parameter ki tarah use karna ----------
    // &impl Animal -> "koi bhi type jo Animal implement karta ho"
    make_sound(&d);
    make_sound(&cow);

    // ---------- Trait object (dyn) -> alag types ek Vec me ----------
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat), Box::new(Cow)];
    for a in &animals {
        println!("dyn -> {}", a.sound());
    }
}

// Kisi bhi Animal ko accept karta hai
fn make_sound(animal: &impl Animal) {
    println!("Ye bola: {}", animal.sound());
}
