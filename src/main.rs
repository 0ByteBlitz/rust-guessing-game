use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number!");
    println!("You have to guess a number between 1 and 100");
    println!("You have maximum 3 chances to guess the number. Good luck!");

    println!("-------------------");

    let secret_number: i32 = rand::thread_rng().gen_range(1..101);

    let mut max_tries: i32 = 1;

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
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

        println!("The secret number was: {}", secret_number);
        println!("-------------------");

        max_tries += 1;

        if max_tries > 3 {
            println!("Maximum tries reached!");
            println!("You lose!");
            break;
        }
    }
}
