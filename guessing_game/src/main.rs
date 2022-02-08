use std::io;
use rand::Rng;

fn main() {
    println!("I'm thinking of a number between 1 and 10...");
    let think = rand::thread_rng().gen_range(1..10);

    println!("Please input your guess.");

    let mut guess_str = String::new();

    io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line");

    match  guess_str.trim_end().parse::<i32>() {
        Ok(guess) => {
            if think == guess {
                println!("correct!");
            } else {
                println!("You guessed: {}, but I was thinking of {}", guess, think);
            }
        },
        Err(e) => println!("ERROR: {}", e),
    }

    
}