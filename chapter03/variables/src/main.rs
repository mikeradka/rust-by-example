fn main() {
    /* 
     * In Rust, variables are immutable by default
     * 
     * let x = 5;
     * println!("The value of x is {x}");
     * x = 6;
     * println!("The value of x is: {x}");
     * 
     * This would result in a compiling error
     * x = 6;
     * ^^^^^ cannot assign twice to immutable variable
     */

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
