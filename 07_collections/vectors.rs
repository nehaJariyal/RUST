// ============================================================
// TOPIC: Vectors (Vec<T>)  (badhne-ghatne wali list)
// ============================================================
// Run:  rustc vectors.rs && ./vectors
// ------------------------------------------------------------
// Vec = same type ki values ka growable (badhne wala) collection.
// Array fixed size ka hota hai, Vec ki size runtime par badal sakti hai.

fn main() {
    // ---------- 1) Vector banana ----------
    let mut nums: Vec<i32> = Vec::new(); // khali vector
    nums.push(10); // end me add
    nums.push(20);
    nums.push(30);
    println!("nums = {:?}", nums);

    // vec! macro se ek saath banana
    let mut fruits = vec!["aam", "seb", "kela"];
    println!("fruits = {:?}", fruits);

    // ---------- 2) Access karna ----------
    // (a) index se -> galat index par PANIC ho jata hai
    println!("Pehla fruit = {}", fruits[0]);
    // (b) .get() -> Option deta hai (safe)
    match fruits.get(10) {
        Some(f) => println!("index 10 = {}", f),
        None => println!("index 10 par kuch nahi (safe)"),
    }

    // ---------- 3) Update / remove ----------
    fruits[1] = "angoor"; // seb -> angoor
    fruits.push("santra");
    let removed = fruits.pop(); // end se nikaalta hai -> Option
    println!("nikala = {:?}, ab fruits = {:?}", removed, fruits);
    fruits.remove(0); // index 0 wala hatao
    println!("index 0 hatane ke baad = {:?}", fruits);

    // ---------- 4) Loop chalana ----------
    for n in &nums {
        println!("num: {}", n);
    }
    // mutable loop -> har value ko badalna
    for n in &mut nums {
        *n *= 2; // dugna
    }
    println!("dugna karne ke baad nums = {:?}", nums);

    // ---------- 5) Useful methods ----------
    println!("length = {}", nums.len());
    println!("khali hai? {}", nums.is_empty());
    println!("20 hai? {}", nums.contains(&20));

    // sum aur max (iterator methods)
    let total: i32 = nums.iter().sum();
    let max = nums.iter().max();
    println!("total = {}, max = {:?}", total, max);

    // ---------- 6) sort ----------
    let mut random = vec![5, 2, 8, 1, 9, 3];
    random.sort();
    println!("sort ke baad = {:?}", random);
}
