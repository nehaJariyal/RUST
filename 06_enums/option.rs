// ============================================================
// TOPIC: Option<T>  (value ho ya na ho - null ka safe tarika)
// ============================================================
// Run:  rustc option.rs && ./option
// ------------------------------------------------------------
// Rust me "null" nahi hota. Uski jagah Option enum use hota hai:
//   enum Option<T> { Some(T), None }
//   Some(value) -> value maujood hai
//   None        -> value nahi hai
// Compiler zabardasti "None" case handle karwata hai -> safe code.

fn main() {
    // ---------- 1) Some aur None ----------
    let kuch: Option<i32> = Some(5);
    let khali: Option<i32> = None;
    println!("kuch = {:?}, khali = {:?}", kuch, khali);

    // ---------- 2) match se value nikalna ----------
    match kuch {
        Some(v) => println!("Value mili: {}", v),
        None => println!("Koi value nahi"),
    }

    // ---------- 3) if let (chhota shortcut) ----------
    if let Some(v) = kuch {
        println!("if let: {}", v);
    }

    // ---------- 4) unwrap_or -> None ho to default value ----------
    println!("khali.unwrap_or(0) = {}", khali.unwrap_or(0));
    println!("kuch.unwrap_or(0)  = {}", kuch.unwrap_or(0));

    // ---------- 5) map -> value ho to transform karo ----------
    let doubled = kuch.map(|x| x * 2); // Some(10)
    println!("map se = {:?}", doubled);

    // ---------- 6) Function jo Option return kare ----------
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

// Naam dhoondo -> mila to Some(index), nahi to None
fn find_name(list: &[&str], target: &str) -> Option<usize> {
    for (i, name) in list.iter().enumerate() {
        if *name == target {
            return Some(i);
        }
    }
    None
}
