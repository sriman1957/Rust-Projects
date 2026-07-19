use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to Number Guessing game");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is : {secret_number}");

    loop {
    // asking user to enter input
        println!("Please enter a number between 1 and 100: ");

        let mut input = String::new();
        
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");
        


        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("your entered input is {input}");

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Your Guess is too small, guess higher"),
            Ordering::Greater => println!("Your Guess is too high, guess lower"),
            Ordering::Equal => {
                println!("You guessed it right, Congratulations!");
                break;
            }
        }
    }
}