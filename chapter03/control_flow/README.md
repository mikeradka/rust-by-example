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