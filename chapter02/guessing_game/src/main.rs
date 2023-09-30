use std::io; // Bring the std::io library into scope

// The main function is the entry point into the program
fn main() { // Declare the main function with no parameters
    println!("Guess the number!");  // println! is a marcro that prints a string to the screen
    println!("Please input your guess: ");

    let mut guess = String::new();  // Create a new mutable empty string

    // Receiving user input
    io::stdin()
        .read_line(&mut guess)  // Call the read_line method on the stdio handle to get input from the user
        .expect("Failed to read line"); // Handle an error if the line cannot be read

    println!("You guessed: {guess}");
}