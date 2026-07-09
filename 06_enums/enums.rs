// ============================================================
// TOPIC: Enums  (one type with many possible "forms")
// ============================================================
// Run:  rustc enums.rs && ./enums
// ------------------------------------------------------------
// Enum = "enumeration". A value that can be one of several variants.
// Each variant can also carry its own data.

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

// Variants that hold different kinds of data
#[derive(Debug)]
enum Message {
    Quit,                       // no data
    Move { x: i32, y: i32 },    // named data like a struct
    Write(String),              // one String
    ChangeColor(u8, u8, u8),    // three values (tuple-like)
}

// You can also write methods on enums (impl)
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

    // ---------- 2) Data-carrying enum + method ----------
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

// Function that acts based on the enum variant
fn move_player(dir: Direction) {
    match dir {
        Direction::North => println!("Upar gaye"),
        Direction::South => println!("Neeche gaye"),
        Direction::East => println!("Dahine gaye"),
        Direction::West => println!("Baayein gaye"),
    }
}
