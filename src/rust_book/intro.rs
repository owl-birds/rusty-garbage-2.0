// use library
use std::io;

pub fn intro_hello(){
    println!("HELLO WORLD!");
}

pub fn guess_the_number(){
    println!("GUess the number Game.");
    println!("Please Input your guess.");
    let mut guess = String::new();
    io::stdin()
       .read_line(&mut guess)
       .expect("Failed to read the input (line)");
    println!("You guessed : {guess}");
}