use std::io;

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
    
    println!("\nFloating-Point Types:");
    
    let my_float_1 = 2.0;
    println!("my_float_1 = {my_float_1:.1} (f64)"); // :.1 gives one decimal precision

    let my_float_2: f32 = 3.0;
    println!("my_float_2 = {my_float_2:.2} (f32)"); // :.2 gives two decimal precision

    println!("\nNumeric Operations:");

    // addition
    let sum = 5 + 10;
    println!("5 + 10 = {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("95.5 - 4.3 = {difference}");

    // division
    let quotient = 56.7 / 32.2;
    println!("56.7 / 32.2 = {quotient}");

    // more division
    let truncated = -5 / 3; // Results in -1
    println!("-5 / 3 = {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("43 % 5 = {remainder}");

    println!("\nThe Boolean Type:");
    // boolean
    let t = true;
    println!("t = {t}");

    let f: bool = false; // with explicit type annotation
    println!("f = {f}");

    println!("\nThe Character Type:");
    let c = 'z';
    println!("char c = {c}");

    let z: char = 'ℤ'; // with explicit type annotation
    println!("char z = {z}");

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat = {heart_eyed_cat}");

    println!("\nThe Tuple Type");
    let my_tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("my_tup = {my_tup:?}");
    let (my_tup_x, my_tup_y, my_tup_z) = my_tup; // use pattern matching to destructure the tuple value
    println!("my_tup_x = {my_tup_x}");  // print the first value in the tuple
    println!("my_tup_y = {my_tup_y}");  // print the second value in the tuple
    println!("my_tup_z = {my_tup_z}");  // print the third value in the tuple

    let five_hundred = my_tup.0;    // Access the first value in the tuple
    println!("my_tup.0 = {five_hundred}");
    let six_point_four = my_tup.1;  // Access the second value in the tuple
    println!("my_tup.1 = {six_point_four}");
    let one = my_tup.2; // Access the third value in the tuple
    println!("my_tup.2 = {one}");

    println!("\nThe Array Type");
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("my_array = {my_array:?}");

    let months = ["January", "February", "March", "April",
    "May", "June", "July", "August", "September", "October", "November", "December"];
    println!("months = {months:?}");

    let five_threes = [3; 5];   // Create an array with 5 elements all with a value of 3
    println!("five_threes = {five_threes:?}");

    let my_array_first = my_array[0];
    println!("my_array[0] = {my_array_first}");

    let my_array_second = my_array[1];
    println!("my_array[1] = {my_array_second}");

    // Example: Invalid Array Element Access
    println!("Please enter an array index (will fail when greater-than 4).");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");
    let element = my_array[index];
    println!("The value of the element at index {index} is: {element}.");

}
