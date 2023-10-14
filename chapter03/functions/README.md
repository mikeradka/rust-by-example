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
- Function bodies are made up of a series of statements optionally ending in an expression
- So far, the functions we've covered haven't included an ending expression.
- Rust is an expression-based language.
  - **Statements** are instructions that perform some action and do _not_ return a value.
  - **Expressions** evaluate to a resultant value.

- Statement example:
```rust
fn main() {
  let y = 6;  // this is a statement
}
```
- Function definitions are also statements. The entire preceding example is a statement in itself.
- Statements do not return values. Therefore, you can't assign a `let` statement to another variable.
- The following code tries to, and will return an error:
```rust
fn main() {
  let x = (let y = 6);
}
```
```
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted
```
- The `let y = 6` statement does not return a value, so there isn't anything for `x` to bind to. This is different than what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment.

- **Expressions** evaluate to a value.
- Consider a math operation, such as `5 + 6`. This is an expression that evalues to the value `11`.
- Expressions can be part of statements. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:
```rust
fn main() {
  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {y}");
}
```

This expression:
```rust
{
  let x = 3;
  x + 1
};
```
is a block that evaluates to `4`. That value gets bound to `y` as part of the `let` statement.
- Note that the `x + 1` line doesn't have a semicolon at the end. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will not return a value.

## Functions with Return Values