# Functions
The [Chapter 3.3 Exercise](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) from [The Rust Programming Language book](https://doc.rust-lang.org/book/).

- Functions are prevalent in Rust code. You’ve already seen one of the most important functions in the language: the `main` function, which is the entry point of many programs. You’ve also seen the `fn` keyword, which allows you to declare new functions.
- Rust code uses _snake case_ as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

```rust
fn main() {
    println!("Hello, world!");

    another_function(); // another_function can be called from main
}

fn another_function() { // another_function is an example of snake case
    println!("Another function.");
}
```
- We define a function in Rust by entering `fn` followed by a function name and a set of parentheses. The curly brackets `{}` tell the compiler where the function body begins and ends.
- We can call any function we’ve defined by entering its name followed by a set of parentheses. Because `another_function` is defined in the program, it can be called from inside the `main` function. 
- Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
- In a Rust program, the lines execute in the order in which they appear in the main function.

## Parameters
- Functions can have _parameters_: special variables that are part of a function's signature.
- Technically, they are called _arguments_, but people tend to use the words _parameter_ and _argument_ interchangeably.
- In this version of *another_function*, we add a parameter:

```rust
fn main() {
  another_function(5);
}

fn another_function(x: i32) { // function has one parameter, x
  println!("The value of x is: {x}");
}
```
- In function signatures, you **must** declare the type of each parameter.
- When defining multiple parameters, separate the parameter declarations with commas, like this:

```rust
fn main() {
  print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}
```

- This example creates a function named *print_labeled_measurement* with two parameters: `value` (`i32`), and `unit_label` (`char`).

```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
The measurement is: 5h
```
- Because we called the function with `5` as the value for `value` and `'h'` as the value for `unit_label`, the program output contains those values.

## Statements and Expressions