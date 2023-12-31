fn main() {
    println!("Hello, world!");

    first_function();

    second_function(5);

    print_labeled_measurement(5, 'h');

    expression_example();

    let z = five();
    println!("The value of z is: {z}");

    let a = plus_one(5);
    println!("The value of a is: {a}");
}

fn first_function() {
    println!("First function!");
}

fn second_function(x: i32) {    // function has 1 parameter, x
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression_example() {
    let y = {   // start the expression
        let x = 3;
        x + 1   // note the lack of semicolon, because this is an expression, not a statement
    };

    println!("The value of y is: {y}")
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}