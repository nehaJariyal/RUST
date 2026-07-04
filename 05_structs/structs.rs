// ============================================================
// TOPIC: Structs  (apna custom data type banana)
// ============================================================
// Run:  rustc structs.rs && ./structs
// ------------------------------------------------------------
// Struct = related data ko ek naam ke andar group karna.
// (Jaise ek "Student" ke naam, age, marks ek saath.)

// #[derive(Debug)] -> taaki hum {:?} se poora struct print kar sake
#[derive(Debug)]
struct Student {
    name: String, // field: type
    age: u32,
    marks: f64,
    is_pass: bool,
}

fn main() {
    // ---------- 1) Struct ka instance banana ----------
    let s1 = Student {
        name: String::from("Neha"),
        age: 21,
        marks: 88.5,
        is_pass: true,
    };

    // Fields ko dot (.) se access karte hai
    println!("Naam: {}", s1.name);
    println!("Age: {}", s1.age);
    println!("Marks: {}", s1.marks);

    // Poora struct debug print
    println!("Pura struct = {:?}", s1);
    // {:#?} -> pretty (sundar, multi-line) print
    println!("Pretty:\n{:#?}", s1);

    // ---------- 2) MUTABLE struct (fields badalna) ----------
    // Pura instance mut hona chahiye tabhi koi field badal sakte hai
    let mut s2 = Student {
        name: String::from("Amit"),
        age: 20,
        marks: 45.0,
        is_pass: false,
    };
    s2.marks = 70.0;    // field update
    s2.is_pass = true;
    println!("Update ke baad: {:?}", s2);

    // ---------- 3) Function se struct banana ----------
    let s3 = build_student(String::from("Priya"), 22);
    println!("build_student se: {:?}", s3);

    // ---------- 4) STRUCT UPDATE syntax (..) ----------
    // Baaki fields kisi aur instance se copy kar lo
    let s4 = Student {
        name: String::from("Rahul"),
        ..s3 // baaki age, marks, is_pass s3 se le lo
    };
    println!("Update syntax se: {:?}", s4);
}

// Function jo Student return karta hai
fn build_student(name: String, age: u32) -> Student {
    Student {
        name, // field aur variable ka naam same ho to chhota likh sakte hai
        age,
        marks: 0.0,
        is_pass: false,
    }
}
