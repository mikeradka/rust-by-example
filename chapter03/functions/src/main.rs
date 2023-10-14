fn main() {
    println!("Hello, world!");

    first_function();

    second_function(5);

    print_labeled_measurement(5, 'h');
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