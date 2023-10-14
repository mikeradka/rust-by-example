fn main() {
    println!("Hello, world!");

    first_function();

    second_function(5);
}

fn first_function() {
    println!("First function!");
}

fn second_function(x: i32) {    // function has 1 parameter, x
    println!("The value of x is: {x}");
}