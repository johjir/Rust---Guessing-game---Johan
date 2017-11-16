extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    let mut guess_tracker: u32 = 0;
    
    let mut guesshash = HashMap::new();     // A hashmap for the guesses is created
    
    //println!("The secret number is {}", secret_number);
    
    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
            
        let guessnum: u32 = match guess.clone().trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Your guess needs to be a number!");
                continue;
            }
        };
        
        guess_tracker += 1;
        guess.pop();
        guesshash.insert(guess_tracker, guess);     // Current guess and its number is pushed onto the hashmap
        
        println!("You guessed: {}", guessnum);
        
        match guessnum.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                for guesselement in ({if guess_tracker>2 {guess_tracker-2} else {1}}..guess_tracker+1).rev() {  // The three last guesses are printed in reverse order with all guesses printed if they are less than 3
                    println!("Guess {} was {}", guesselement, guesshash.get(&guesselement).unwrap());
                }
                println!("Total number of guesses was {}", guess_tracker);
                break;
            }    
        }
        
        println!("You have guessed {} times\n", guess_tracker);
    }
}
