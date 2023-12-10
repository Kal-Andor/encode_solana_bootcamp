
fn main() {
    println!("Welcome to my first Rust project!");
    fizz_buzz()
}

fn fizz_buzz() {
    let mut counter:i32 =0;
    for n in 1..=301 {
        let word:Option<&str> =
            if n % 15 == 0 { Some("fizz-buzz") }
            else if n % 3 == 0 { Some("fizz") }
            else if n % 5 == 0 { Some("buzz") }
            else { None };
        match word {
            Some("fizz-buzz") => {println!("fizz-buzz"); counter += 1}
            Some(result) => {println!("{}", result)}
            None => {}
        }
    }
    println!("Number of 'fizz-buzz'-s:{}", counter)
}