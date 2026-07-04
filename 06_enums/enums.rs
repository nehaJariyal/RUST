// ============================================================
// TOPIC: Enums  (ek type jiske kai possible "roop" ho)
// ============================================================
// Run:  rustc enums.rs && ./enums
// ------------------------------------------------------------
// Enum = "enumeration". Ek value jo kai variants me se ek ho sakti hai.
// Har variant apne saath data bhi rakh sakta hai.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Variants jo alag alag tarah ka data rakhte hai
#[derive(Debug)]
enum Message {
    Quit,                       // koi data nahi
    Move { x: i32, y: i32 },    // struct jaisa named data
    Write(String),              // ek String
    ChangeColor(u8, u8, u8),    // teen values (tuple jaisa)
}

// Enum ke saath bhi methods likh sakte hai (impl)
impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("Band karo"),
            Message::Move { x, y } => format!("Chalo ({}, {}) par", x, y),
            Message::Write(text) => format!("Likho: {}", text),
            Message::ChangeColor(r, g, b) => format!("Color: {},{},{}", r, g, b),
        }
    }
}

fn main() {
    // ---------- 1) Simple enum ----------
    let dir = Direction::West;
    println!("Direction = {:?}", dir);
    move_player(Direction::North);
    move_player(dir);

    // ---------- 2) Data wale enum + method ----------
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Namaste")),
        Message::ChangeColor(255, 0, 128),
    ];

    for m in &messages {
        println!("{:?} -> {}", m, m.describe());
    }
}

// Function jo enum ke variant ke hisaab se kaam kare
fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Upar gaye"),
        Direction::South => println!("Neeche gaye"),
        Direction::East => println!("Dahine gaye"),
        Direction::West => println!("Baayein gaye"),
    }
}
