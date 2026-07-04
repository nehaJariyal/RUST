// ============================================================
// TOPIC: Constants in Rust  (const aur static ka use)
// ============================================================
// Run:  rustc constants.rs && ./constants
// ------------------------------------------------------------

// CONST -> ye compile time par fix ho jate hai.
// Rules:
//   * naam hamesha UPPERCASE me likhte hai (convention)
//   * type batana ZARURI hai (yaha : f64 , : &str)
//   * `mut` allowed nahi hai, value kabhi change nahi hoti
//   * kisi bhi scope me bana sakte hai (global bhi, function ke andar bhi)
const PI: f64 = 3.14159;
const COMPANY_NAME: &str = "Rust Learners";

// STATIC -> ye const jaisa hi hai par iski ek fixed memory location hoti hai
// (program ki puri life tak same address par rehta hai).
static GREETING: &str = "Namaste";

fn main() {
    // Global const ko seedha use kar sakte hai
    println!("{} se {} ka course", COMPANY_NAME, PI);
    println!("{}!", GREETING);

    // Function ke andar bhi const bana sakte hai
    const MAX_STUDENTS: u32 = 60;
    println!("Ek class me max {} students", MAX_STUDENTS);

    // Const ka use ek calculation me
    let radius = 5.0;
    let area = PI * radius * radius; // circle ka area = PI * r^2
    println!("Circle ka area = {}", area);

    // NOTE: neeche wali line ERROR degi kyunki const change nahi hota
    // PI = 3.14;   // <-- cannot assign to this expression
}
