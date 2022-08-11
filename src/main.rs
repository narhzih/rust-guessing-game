use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    println!("This is just a guessing game");

    loop {
        let mut guess: String = String::new();
        println!("Please enter your guess here:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to realine for some reasons");
        println!("Your guess is {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You jut won big!!!");
                break;
            }
        }
    }
}
