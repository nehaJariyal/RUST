// ============================================================
// TOPIC: Vectors (Vec<T>)  (growable list)
// ============================================================
// Run:  rustc vectors.rs && ./vectors
// ------------------------------------------------------------
// Vec = a growable collection of values of the same type.
// Array has fixed size; Vec's size can change at runtime.

fn main() {
    // ---------- 1) Creating a vector ----------
    let mut nums: Vec<i32> = Vec::new(); // empty vector
    nums.push(10); // add at the end
    nums.push(20);
    nums.push(30);
    println!("nums = {:?}", nums);

    // create all at once with vec! macro
    let mut fruits = vec!["aam", "seb", "kela"];
    println!("fruits = {:?}", fruits);

    // ---------- 2) Accessing elements ----------
    // (a) by index -> PANIC on invalid index
    println!("Pehla fruit = {}", fruits[0]);
    // (b) .get() -> returns Option (safe)
    match fruits.get(10) {
        Some(f) => println!("index 10 = {}", f),
        None => println!("index 10 par kuch nahi (safe)"),
    }

    // ---------- 3) Update / remove ----------
    fruits[1] = "angoor"; // seb -> angoor
    fruits.push("santra");
    let removed = fruits.pop(); // removes from end -> Option
    println!("nikala = {:?}, ab fruits = {:?}", removed, fruits);
    fruits.remove(0); // remove element at index 0
    println!("index 0 hatane ke baad = {:?}", fruits);

    // ---------- 4) Looping ----------
    for n in &nums {
        println!("num: {}", n);
    }
    // mutable loop -> change each value
    for n in &mut nums {
        *n *= 2; // double
    }
    println!("dugna karne ke baad nums = {:?}", nums);

    // ---------- 5) Useful methods ----------
    println!("length = {}", nums.len());
    println!("khali hai? {}", nums.is_empty());
    println!("20 hai? {}", nums.contains(&20));

    // sum and max (iterator methods)
    let total: i32 = nums.iter().sum();
    let max = nums.iter().max();
    println!("total = {}, max = {:?}", total, max);

    // ---------- 6) sort ----------
    let mut random = vec![5, 2, 8, 1, 9, 3];
    random.sort();
    println!("sort ke baad = {:?}", random);
}
