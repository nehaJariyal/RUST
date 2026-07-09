// ============================================================
// TOPIC: Structs  (creating your own custom data type)
// ============================================================
// Run:  rustc structs.rs && ./structs
// ------------------------------------------------------------
// Struct = grouping related data under one name.
// (Like putting a "Student"'s name, age, and marks together.)

// #[derive(Debug)] -> so we can print the whole struct with {:?}
#[derive(Debug)]
struct Student {
    name: String, // field: type
    age: u32,
    marks: f64,
    is_pass: bool,
}

fn main() {
    // ---------- 1) Creating a struct instance ----------
    let s1 = Student {
        name: String::from("Neha"),
        age: 21,
        marks: 88.5,
        is_pass: true,
    };

    // Access fields with dot (.)
    println!("Naam: {}", s1.name);
    println!("Age: {}", s1.age);
    println!("Marks: {}", s1.marks);

    // Debug print the whole struct
    println!("Pura struct = {:?}", s1);
    // {:#?} -> pretty (multi-line) print
    println!("Pretty:\n{:#?}", s1);

    // ---------- 2) MUTABLE struct (changing fields) ----------
    // The whole instance must be mut to change any field
    let mut s2 = Student {
        name: String::from("Amit"),
        age: 20,
        marks: 45.0,
        is_pass: false,
    };
    s2.marks = 70.0;    // field update
    s2.is_pass = true;
    println!("Update ke baad: {:?}", s2);

    // ---------- 3) Creating a struct via a function ----------
    let s3 = build_student(String::from("Priya"), 22);
    println!("build_student se: {:?}", s3);

    // ---------- 4) STRUCT UPDATE syntax (..) ----------
    // Copy the remaining fields from another instance
    let s4 = Student {
        name: String::from("Rahul"),
        ..s3 // take age, marks, is_pass from s3
    };
    println!("Update syntax se: {:?}", s4);
}

// Function that returns a Student
fn build_student(name: String, age: u32) -> Student {
    Student {
        name, // shorthand when field and variable have the same name
        age,
        marks: 0.0,
        is_pass: false,
    }
}
