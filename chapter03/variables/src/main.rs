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

    /*
     * Shadowing
     * You can declare a new variable with the same name
     * as a previous variable. The first variable is
     * 'shadowed' by the second. The second variable
     * 'overshadows'the first. A variable can be shadowed
     * by using the same name.
     */

    let y = 5;  // y is bound to a value of 5

    let y = y + 1;  // new variable y is created, overshadowing the first y

    // Create an inner scope with curly brackets
    {
        let y = y * 2;  // third y overshadows second y
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}"); // second y overshadows first y

    let spaces = "   ";
    println!("First 'spaces' var: {spaces} (str)");
    let spaces = spaces.len();
    println!("Second 'spaces' var: {spaces} (int)");

}
