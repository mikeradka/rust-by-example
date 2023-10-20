# Control Flow
- The ability to run some code depending on whether a condition is `true` and to run some code repeatedly while a condition is `true` are basic building blocks in most programming languages.
- The most common constructs that let you control the flow of execution of Rust code are `if` expressions and loops.

## `if` Expressions
- An `if` expression allows you to branch your code depending on conditions.
- You provide a condition and then state, _"If this condition is met, run this block of code. If the condition is not met, do not run this block of code."_

```rust
fn main() {
  let number = 3;

  if number < 5 {
    println!("condition was true");
  }
  else {
    println!("condition was false");
  }
}
```

- All `if` expressions start with the keyword `if`, followed by a condition. In
This case, the condition checks whether or not the variable `number` has a value less than `5`. We place the block of code to execute if the condition is `true` immediately after the condition inside curly brackets. Blocks of code associated
with the conditions in `if` expressions are sometimes called `arms`.
- Optionally, we can also include an `else` expression, which we chose to do here,
to give the program an alternative block of code to execute should the condition
evaluate to `false`
- If you don't provide an `else` expression and the condition is `false`, the program will just skip the `if` block and move on to the next bit of code.

- If we change the value of `number` to one that makes the condition `false`:
```rust
let number = 7;
```
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
condition was false
```

- It's also worth noting that the condition in this code _must_ be a `bool`. If the condition isn't a `bool`, there will be an error:

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

- The `if` condition evaluates to a value of `3` this time, and Rust throws an error:
```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

- The error indicates that Rust expected a `bool` but got an integer. Unlike languages such as Ruby and Javascript, Rust will not automatically try to convert non-Boolean types to a Boolean. You must always be explicit and always provide `if` with a Boolean as a condition.
- If we want the `if` code block to run only when a number is _not equal_ to `0`, for example, we can change the `if` expression to the following:

```rust
fn main() {
  let number = 3;

  if number != 0 {
    println!("number was something other than 0.")
  }
}
```
- Running this code will print `number was something other than 0.`