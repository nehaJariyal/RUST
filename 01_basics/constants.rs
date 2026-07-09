// ============================================================
// TOPIC: Constants in Rust  (using const and static)
// ============================================================
// Run:  rustc constants.rs && ./constants
// ------------------------------------------------------------

// CONST -> these are fixed at compile time.
// Rules:
//   * name is always written in UPPERCASE (convention)
//   * type is REQUIRED (here : f64 , : &str)
//   * `mut` is not allowed, value never changes
//   * can be created in any scope (global or inside a function)
const PI: f64 = 3.14159;
const COMPANY_NAME: &str = "Rust Learners";

// STATIC -> similar to const but has a fixed memory location
// (stays at the same address for the entire life of the program).
static GREETING: &str = "Namaste";

fn main() {
    // Global const can be used directly
    println!("{} se {} ka course", COMPANY_NAME, PI);
    println!("{}!", GREETING);

    // You can also create const inside a function
    const MAX_STUDENTS: u32 = 60;
    println!("Ek class me max {} students", MAX_STUDENTS);

    // Using const in a calculation
    let radius = 5.0;
    let area = PI * radius * radius; // circle area = PI * r^2
    println!("Circle ka area = {}", area);

    // NOTE: the line below will ERROR because const cannot be changed
    // PI = 3.14;   // <-- cannot assign to this expression
}
