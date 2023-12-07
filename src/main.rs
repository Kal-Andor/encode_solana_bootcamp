
fn main() {
    println!("Welcome to my first Rust project!");
    fizz_buzz()
}

fn fizz_buzz() {
    for n in 1..=301 {
        match n % 3 {
            0 => println!(" fizz {}", &n),
            _ => {}
        }
    }
}

