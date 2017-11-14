extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Function requesting user input and returning a u32 or an error string
fn userinput() -> Result<u32, String>{

    let mut guess = String::new();
        
    io::stdin().read_line(&mut guess)           // Requests user input, stored in guess
        .expect("Failed to read line");
            
    let guess: u32 = match guess.trim().parse() {       // Attempts to parse guess to a u32
        Ok(num) => num,
        Err(_)  => return Err(String::from("your guess needs to be a number")), // Returns an error str otherwise 
    };
    Ok(guess)

}

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    let mut guess_tracker = 0;
    let mut guess: u32;
    
    //println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        
        match userinput() {
            Ok(g) => guess = g,
            Err(err) => {
                println!("in parsing u32, {}", err);
                continue;
            }
        };
        
        guess_tracker += 1;
        
        println!("You guessed: {}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                println!("Total number of guesses was {}", guess_tracker);
                break;
            }    
        }
        
        println!("You have guessed {} times\n", guess_tracker);
    }
}
