use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Let's a play a number guessing game!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please type your guess, and then press enter.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
