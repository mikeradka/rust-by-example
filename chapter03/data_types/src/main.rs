fn main() {
    /*
     * Example of a data type annotation
     * 
     * let guess = "42".parse().expect("Not a number!");
     *  
     * Running this without a type annotation will result
     * in an error:
     * 
     * error[E0282]: type annotations needed
     * --> src/main.rs:2:9
     * |
     * 2 |     let guess = "42".parse().expect("Not a number!");
     *   |         ^^^^^
     * 
     * In this case, the compiler needs to know which data
     * type we want to use.
     */
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess = {guess}");
}
