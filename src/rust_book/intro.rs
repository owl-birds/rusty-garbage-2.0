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

pub fn common_progamming_concepts(){
    //variables are immutable by default
    println!("COMMON PROGRAMMING CONCEPTS");
    // let x = 5;
    let mut x = 5;
    println!("the value of x is : {x}");
    x = 6;
    println!("the value of x is : {x}");

    // CONSTANTS
    // cant use mut with constant
    // always immutable
    const ONE_HOUR_IN_SECONDS: u32 = 1 * 60 * 60;
    println!("{ONE_HOUR_IN_SECONDS}");

    // SHADOWING
    println!("SHADOWING");
    let x = x + 1000; // SAHOWING
    // you can declare a new variable with 
    // the same name as prev vars
    println!("{x}");
    // let mut x = x + 1000;
    // println!("{x}");
    let two_spaces= "  ";
    println!("{two_spaces}");
    let two_spaces = two_spaces.len();
    println!("{two_spaces}");
    let number = if 90 > 9 {1000} else {2000};
    loop {
        println!("LOOP");
        break;
    }
    // return a value from loop
    let mut counter = 0;
    let result = loop{
        counter += 1;
        if counter == 100{
            break counter; 
            // returning value from loop
        }
    };
    println!("{result}");
    // loop labels to disambiguate between multiple loops
}