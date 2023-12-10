const FIZZ:&str = "fizz";
const BUZZ:&str = "buzz";
const FIZZ_BUZZ:&str = "fizz-buzz";

fn main() {
    println!("Welcome to my first Rust project!");
    fizz_buzz()
}

fn fizz_buzz() {
    let mut counter:i32 =0;
    for n in 1..=301 {
        let word:Option<&str> =
            if n % 15 == 0 { Some(FIZZ_BUZZ) }
            else if n % 3 == 0 { Some(FIZZ) }
            else if n % 5 == 0 { Some(BUZZ) }
            else { None };
        match word {
            Some(FIZZ_BUZZ) => {println!("{}", FIZZ_BUZZ); counter += 1}
            Some(result) => {println!("{}", result)}
            None => {}
        }
    }
    println!("Number of 'fizz-buzz'-s:{}", counter)
}

