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

    println!("\nInteger Types:");

    let my_decimal_1 = 98221;
    println!("my_decimal_1 = {my_decimal_1} (i32)");

    let my_decimal_2 = 98_222;  // the underscore can be used to more easily read
    println!("my_decimal_2 = {my_decimal_2} (i32)");

    let my_hex = 0xff;  // hex 0xff = 255
    println!("my_hex = {my_hex} (i32)");

    let my_octal = 0o77; // 63
    println!("my_octal = {my_octal} (i32)");

    let my_binary = 0b1111_0000; // 240
    println!("my_binary = {my_binary} (i32)");

    let my_byte = b'A';
    println!("my_byte = {my_byte} (u8)");
    println!();
}
