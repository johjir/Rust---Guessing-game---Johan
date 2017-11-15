extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1,101);
    
    let mut guess_tracker: u32 = 0;
    
    let mut guessvec: Vec<(u32, String)> = Vec::new();          // A vector for the guesses is created
    
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
        
        guess_tracker = guess_tracker + 1;
        
        guess.pop();
        guessvec.push((guess_tracker, guess));          // Current guess and its number is pushed onto the vector
        if guessvec.len() > 3 {                         // Removes the earliest guesses when there are more than 3
            guessvec = guessvec[1..].to_vec();
        }
        
        println!("You guessed: {}", guessnum);
        
        match guessnum.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                guessvec.reverse();                                 // Reverses order of the vector elements
                for guesselement in guessvec {
                    println!("Guess {} was {}", guesselement.0, guesselement.1);    // All guesses are printed out 
                }
                println!("Total number of guesses was {}", guess_tracker);
                break;
            }    
        }
        
        println!("You have guessed {} times\n", guess_tracker);
    }
}
