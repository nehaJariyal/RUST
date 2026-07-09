// ============================================================
// TOPIC: Option<T>  (value may or may not exist - safe alternative to null)
// ============================================================
// Run:  rustc option.rs && ./option
// ------------------------------------------------------------
// Rust has no "null". Instead it uses the Option enum:
//   enum Option<T> { Some(T), None }
//   Some(value) -> value exists
//   None        -> no value
// The compiler forces you to handle the "None" case -> safe code.

fn main() {
    // ---------- 1) Some and None ----------
    let kuch: Option<i32> = Some(5);
    let khali: Option<i32> = None;
    println!("kuch = {:?}, khali = {:?}", kuch, khali);

    // ---------- 2) Extracting value with match ----------
    match kuch {
        Some(v) => println!("Value mili: {}", v),
        None => println!("Koi value nahi"),
    }

    // ---------- 3) if let (short shortcut) ----------
    if let Some(v) = kuch {
        println!("if let: {}", v);
    }

    // ---------- 4) unwrap_or -> default value when None ----------
    println!("khali.unwrap_or(0) = {}", khali.unwrap_or(0));
    println!("kuch.unwrap_or(0)  = {}", kuch.unwrap_or(0));

    // ---------- 5) map -> transform the value if it exists ----------
    let doubled = kuch.map(|x| x * 2); // Some(10)
    println!("map se = {:?}", doubled);

    // ---------- 6) Function that returns Option ----------
    let names = ["Neha", "Amit", "Priya"];
    match find_name(&names, "Amit") {
        Some(i) => println!("'Amit' index {} par mila", i),
        None => println!("Nahi mila"),
    }
    println!("Rahul: {:?}", find_name(&names, "Rahul"));

    // ---------- 7) is_some / is_none ----------
    println!("kuch.is_some() = {}", kuch.is_some());
    println!("khali.is_none() = {}", khali.is_none());
}

// Search for a name -> Some(index) if found, None otherwise
fn find_name(list: &[&str], target: &str) -> Option<usize> {
    for (i, name) in list.iter().enumerate() {
        if *name == target {
            return Some(i);
        }
    }
    None
}
