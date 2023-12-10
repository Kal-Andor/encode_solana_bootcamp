
fn main() {
    println!("Welcome to my first Rust project!");
    fizz_buzz()
}

fn fizz_buzz() {
    for n in 1..=301 {
        // match n % 3 {
        //     0 => println!(" fizz {}", &n),
        //     _ => {}
        // }
        let word = if n % 15 == 0 { "fizz-buzz" } else if n % 3 == 0 { "fizz" } else if n % 5 == 0 { "buzz" } else { "" };
        println!("{} {}", word, n)
    }
}

