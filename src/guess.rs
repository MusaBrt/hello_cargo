use std::{cmp::Ordering, io};
use rand::Rng;

#[allow(dead_code)]
pub fn start_guess() -> () {
    println!("Now... I'm thinking a number between 0 and 100. And you guess the number okay?");

    // threadrng mut because the gen_range fn is internally modifies the threadrng struct
    let mut threadrng = rand::thread_rng();
    let mut random_number = threadrng.gen_range(0..=100);

    let mut entry = String::new();
    loop {
        print!("> ");
        io::stdin().read_line(&mut entry).expect("An error occur when read the entry.");
        if entry.eq("quit") { println!("Bye bye ^^."); break; }

        // look at the match. its comparing the parse's result with Ok and Err. 
        // u8 because unsigned 8 bit -> 0-256
        let entry: u8 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Only number mate... Only numbers.");
                continue;
            }
        };
        // there is a semicolon because we're assigning a value to variable of named as entry

        match entry.cmp(&random_number) {
            Ordering::Equal => {
                println!("Good job mate! Number was: {}. Let's play one more.", random_number);
                random_number = threadrng.gen_range(0..=100);
            }
            Ordering::Less => println!("I think your guess is feel short..."),
            Ordering::Greater => println!("Your guess so strong!"),
        }
        
    }
}