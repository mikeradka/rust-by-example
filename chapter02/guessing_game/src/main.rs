use rand::Rng;  // Bring the Rng trait into scope
use std::cmp::Ordering; // Bring the Ordering type into scope
use std::io; // Bring the std::io library into scope

// The main function is the entry point into the program
fn main() { // Declare the main function with no parameters
    println!("Guess the number!");  // println! is a marcro that prints a string to the screen

    // Generate a secret number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Print the secret number
    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess: ");

        // Create a new mutable empty string
        let mut guess = String::new();
    
        // Receiving user input
        io::stdin()
            .read_line(&mut guess)  // Call the read_line method on the stdio handle to get input from the user
            .expect("Failed to read line"); // Handle an error if the line cannot be read
    
        // convert guess to an unsigned 32-bit integer
        let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
        println!("You guessed: {guess}");
    
        // A match expression decides what to do next based on which variant of Ordering is returned
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),   // If the value of guess is lower than secret number, print 'Too small'
            Ordering::Greater => println!("Too big!"),   // If the value of guess is higher than secret number, print 'Too big'
            Ordering::Equal => {
                println!("You win!");   // If the value of guess equals the secret number, print 'You win!'
                break;  // Exit the loop
            }
        }
    }
}