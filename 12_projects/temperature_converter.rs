// ============================================================
// PROJECT: Temperature Converter  (Celsius <-> Fahrenheit <-> Kelvin)
// ============================================================
// Run:  rustc temperature_converter.rs && ./temperature_converter
// ------------------------------------------------------------
// Concepts used: functions, loops, formatting, a bit of math.

// Celsius -> Fahrenheit :  F = (C * 9/5) + 32
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

// Fahrenheit -> Celsius :  C = (F - 32) * 5/9
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

// Celsius -> Kelvin :  K = C + 273.15
fn celsius_to_kelvin(c: f64) -> f64 {
    c + 273.15
}

fn main() {
    println!("===== Temperature Converter =====\n");

    // ---------- 1) A single conversion ----------
    let temp_c = 37.0;
    println!(
        "{}°C = {:.1}°F = {:.2}K",
        temp_c,
        celsius_to_fahrenheit(temp_c),
        celsius_to_kelvin(temp_c)
    );

    // {:.1} means up to 1 decimal place, {:.2} means 2 decimals

    // ---------- 2) Build a table (with a loop) ----------
    println!("\nCelsius  ->  Fahrenheit");
    println!("------------------------");
    // from 0 to 100, in steps of 20
    let mut c = 0.0;
    while c <= 100.0 {
        println!("{:>6.1}   ->  {:>7.1}", c, celsius_to_fahrenheit(c));
        c += 20.0;
    }

    // ---------- 3) Reverse check (back to Celsius) ----------
    let f = 98.6;
    println!("\n{}°F waapas Celsius me = {:.1}°C", f, fahrenheit_to_celsius(f));

    // ---------- 4) Freezing / boiling points ----------
    println!("\nPaani ke important points:");
    println!("Freezing: 0°C  = {}°F", celsius_to_fahrenheit(0.0));
    println!("Boiling : 100°C = {}°F", celsius_to_fahrenheit(100.0));
}
