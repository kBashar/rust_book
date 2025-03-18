use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to gussing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Guess a number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("An error hapend, could not read the number");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You have guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("Congratulations.........!");
                break;
            }
        }
    }
}