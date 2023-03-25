use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the Number!");

    let min: u32 = 1;
    let max: u32 = 100;
    let secret_number = rand::thread_rng().gen_range(min..=max);

    let mut total_guesses: u64 = 0;

    loop {
        println!("Please input your guess.\n");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                total_guesses += 1;
                num
            }
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small! Try a bigger number!"),
            Ordering::Greater => println!("Too big! Try a smaller number!"),
            Ordering::Equal => {
                println!("The number was {secret_number}! You Win!");
                println!("Total Guesses: {total_guesses}");
                break;
            }
        }
        
    }
}
