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
  } else {
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

### Handling Multiple Conditions with `else if`

- You can use multiple conditions by combining `if` and `else` in an `else if` expression. For example:

```rust
fn main() {
  let number = 6;

  if number % 4 == 0 {
    println!("number is divisible by 4.");
  } else if number % 3 == 0 {
    println!("number is divisible by 3.");
  } else if number % 2 == 0 {
    println!("number is divisible by 2.");
  } else {
    println!("number is not divisible by 4, 3, or 2.");
  }
}
```

- This program has 4 possible paths it can take. After running it, you should see the following output:

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
number is divisible by 3
```

- When this program executes, it checks each `if` expression in turn and executes the first body for which the condition evaluates to `true`.
- Note that eventhough 6 is divisible by 2, we don't see the output `number is divisible by 2`, nor do we see the `number is not divisible by 4, 3, or 2` text from the `else` block. That's because Rust only executes the block for the first `true` condition, and once it finds one, it doesn't even check the rest.

- Using too many `else if` expressions can clutter your code, so if you have more than one, you might want to _refactor_ your code. Chapter 6 describes a powerful Rust branching construct called `match` for these cases.

### Using `if` in a `let` Statement
- Because `if` is an _expression_, we can use it on the right side of a `let` statement to assign the outcome to a variable.

```rust
fn main() {
  let condition = true;
  let number = if condition { 5 } else { 6 };

  println!("The value of number is: {number}");
}
```
- The `number` variable will be bound to a value based on the outcome of the `if` expression. Running the code looks like this:
```rust
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
The value of number is: 5
```
- Blocks of code evaluate to the last expression in them, and numbers by themselves are also expressions.
- In this case, the valud of the whole `if` expression depends on which block of code executes. 
- This means the values that have the potential to be results from each arm of the `if` **must be the same type**.
- If the types are mismatched, there will be an error:l

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

- The error is thrown, and Rust indicates exactly where to find the problem in the program:

```
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

- The expression in the `if` block evaluates to an integer, and the expression in the `else` block evaluates to a string. This won't work because variables much have a single type, and Rust needs to know at compile time what type the `number` variable is, definitively.
- Knowing the type of `number` lets the compiler verify the type is valid everywhere we use it. Rust wouldn't be able to do that if the type of `number` was only determined at runtime.

### Repetition with Loops
It's often useful to execute a block of code more than once. 
For this task, Rust provides several _loops_, which will run through the code inside the loop body to the end and then 
start immediately back at the beginning. To experiment with 
loops, let's make a new project called loops.
```rust
fn main() {
    loop {
        println!("again!");
    }
}
```
When we run this program, we'll see `again!` printed over and 
over continuously until we stop the program manually. Most 
terminals support the keyboard shortcut _ctrl-c_ to interrupt 
a program that is stuck in a continual loop.
```
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

* Fortunately, Rust also provides a way to break out of a loop 
using code. 
* You can place the `break` keyword within the loop 
to tell the program when to stop executing the loop.
* Also `continue` in a loop tells the program to skip over 
any remaining code in this iteration of the loop and go to 
the next iteration.

#### Returning Values from Loops
One of the uses of a `loop` is to retry an operation you know 
might fail. such as checking whether a thread has completed 
its job.

You might also need to pass the result of that operation out 
of the loop to the rest of your code. To do this, you can add 
the value you want returned after the `break` expression you 
use to stop the loop; that value will be returned out of the 
loop so you can use it, as shown here:

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

Before the loop, we declare a variable named `counter` and initialize 
it to `0`. Then we declare a variable named `result` to hold the the 
value returned from the loop. On every iteration of the loop, we add 
`1` to the `counter` variable, and then check whether the `counter` 
is equal to `10`. When it is, we use the `break` keyword with the value 
`counter * 2`. After the loop, we use a semicolon to end the statement 
that assigns the value to `result`. Finally, we print the value in 
`result`, which in this case is `20`.

#### Loop Labels to Disambiguate Between Multiple Loops
If you have loops within loops, `break` and `continue` apply to the 
innermost loop at that point. You can optionally specify a _loop label_
on a loop that you can use with `break` or `continue` to specify that 
those keywords apply to the labeled loop instead of the innermost loop. 
Loop labels must begin with a single quote. Here is an example with two 
nested loops:

```rust
fn main() {
  let mut count = 0;
  'counting_up: loop {
    println!("count = {count}");
    let mut remaining = 10;

    loop {
      println!("remaining = {remaining}");
      if remaining == 9 {
        break;
      }
      if count == 2 {
        break 'counting_up;
      }
      remaining -= 1;
    }

    count += 1;
  }
  println!("End count = {count}");
}
```