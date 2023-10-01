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
    println!("The first value of mutable x is: {x}");
    x = 6;
    println!("The second value of mutable x is: {x}");
    
    /*
     * Constants aren't just immutable by default,
     * they're always immutable. They are declared
     * by using the let keyword and the type of value
     * must be annotated.
     * Constants may be set only to a constant expression,
     * not the result of a value that can only be
     * computed at runtime.
     */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constant THREE_HOURS_IN_SECONDS: {THREE_HOURS_IN_SECONDS}");
}
