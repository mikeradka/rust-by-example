use std::io; // Bring the std::io library into scope

// The main function is the entry point into the program
fn main() { // Declare the main function with no parameters
    println!("Guess the number!");  // println! is a marcro that prints a string to the screen
    println!("Please input your guess: ");

    let mut guess = String::new();  // Create a new mutable empty string

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}