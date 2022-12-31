// use library
use std::io;
use std::cmp::Ordering;

// downloaded library
use rand::{Rng, random};

pub fn intro_hello(){
    println!("HELLO WORLD!");
}

pub fn guess_the_number(){
    println!("GUess the number Game.");
    let max_number: i32 = 1000;
    let min_number: i32 = 900;
    let random_number: i32 = rand::thread_rng().gen_range(min_number..=max_number);
    loop {    
        println!("Please Input your guess. ({min_number} - {max_number})");
        let mut guess: String  = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input (line)");
        // below is shadowing in rust
        // rust allow us to reuse the guess
        // variable name rather than forcing
        // us to create two unique variables
        
        // let guess: i32 = guess.trim().parse().expect("Please input a number");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        //  The trim method eliminates \n or \r\n
        //  The trim method eliminates \n or \r\n
        // & sign is a referencea
        println!("The secret number : {random_number}");
        println!("You guessed : {guess}");
        match guess.cmp(&random_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win, you guessed the right number");
                break;
            }
        }
    }   
}
// cargo doc --open :: cargo will build
// documentation provided by all of 
// your depedencies locally

