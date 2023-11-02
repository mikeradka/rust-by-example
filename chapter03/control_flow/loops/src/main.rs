fn main() {
    /* Infinite loop example
    loop {
        println!("again!");
    }
    */

    // Returning values from loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}.");
}