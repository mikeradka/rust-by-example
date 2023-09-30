use std::io; // Bring the std::io library into scope
use rand::Rng;  // Bring the Rng trait into scope

// The main function is the entry point into the program
fn main() { // Declare the main function with no parameters
    println!("Guess the number!");  // println! is a marcro that prints a string to the screen

    // Generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Print the secret number
    println!("The secret number is {secret_number}");

    println!("Please input your guess: ");

    // Create a new mutable empty string
    let mut guess = String::new();

    // Receiving user input
    io::stdin()
        .read_line(&mut guess)  // Call the read_line method on the stdio handle to get input from the user
        .expect("Failed to read line"); // Handle an error if the line cannot be read

    println!("You guessed: {guess}");
}