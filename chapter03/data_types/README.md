# Data Types
The [Chapter 3.2 Exercise](https://doc.rust-lang.org/book/ch03-02-data-types.html) from [The Rust Programming Language book](https://doc.rust-lang.org/book/).

## Additional Notes
### Scalar Data Types
A _scalar_ data type represents a single value. Rust has four primary scalar types:
 1. [Integers](#integer-types)
 1. [Floating-point numbers](#floating-point-types)
 1. [Booleans](#the-boolean-type)
 1. [Characters](#the-character-type)
#### Integer Types
- A number *without* a fractional component. - For example, the `u32` is an *unsigned 32-bit integer* (takes up 32 bits of space)
- Any of the below types can be used in Rust to declare the type of integer value:

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

- **Unsigned** integer types start with a `u`, and can be *positive or negative*.
-  **Signed** integer types start with an `i`, and can *only be positive*. Signed numbers are stored using [two's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation. Basically, an `i8` can store numbers from **-128 to 127** and a `u8` can store numbers from **0 to 255**.
- The `isize` and `usize` types depend on the architecture of the computer your program is running on.

| Number Literals  | Example     |
|------------------|-------------|
| Decimal          | 98_222      |
| Hex              | 0xff        |
| Octal            | 0o77        |
| Binary           | 0b1111_0000 |
| Byte (`u8` only) | b'A'        |

- In Rust, integer types default to `i32`. The primary situation in which you'd use `isize` or `usize` is when indexing some sort of collection.

#### Floating-Point Types
* Rust has two primitive types for *floating-point numbers*, which are numbers with decimal points. 
* Rust's floating types are `f32` and `f64`.
* The default type is `f64`.
* All floating-point types are signed
* Example:
```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
* Floating-point numbers are represented according to the IEEE-754 standard. The f32 type is a single-precision float, and f64 has double precision.

#### Numeric Operations
* Rust supports basic mathematical operations like addition, subtraction, multiplecation, division, and remainder.

```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
```

* A list of all operators that Rust provides is [listed here](https://doc.rust-lang.org/book/appendix-02-operators.html).

#### The Boolean Type
- Just like other programming languages, a boolean in Rust has two possible values: `true` and `false`.
- Booleans are one byte in size.
- Booleans in Rust are specified using `bool`.
```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```
- Boolean values are mainly used through conditionals, such as an `if` expression.

#### The Character Type
- Rust's `char` type is the language's most primitive alphabetic type.

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}
```
- `char` literals are specified with single quotes, whereas string literals are specified with double quotes.
- Rust's `char` type is four bytes in size.
- It represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

### Compound Data Types
_Compound data types_ can group _multiple values_ into one type. Rust has two primitive compound types:
1. [Tuples](#the-tuple-type)
2. [Arrays](#the-array-type)

#### The Tuple Type
- A _tuple_ is a general way of grouping together a number of values with a variety of types into a compund data type.
- Tuples have a fixed length. Once declared, they cannot grow or shrink in size.
- A tuple is created by writing a comma-separated list of values inside parenthesis. Each position in the tuple has a type, which do not have to be the same.

```rust
fn main() {
    let my_tup: (i32, f64, u8) = (500, 6.4, 1);
}
```
- The variable `my_tup` _binds_ to the entire tuple, because it is considered a _single compound element_.
- To get the individual values out of a tuple, we can use _pattern matching_ to _destructure_ a tuple value:

```rust
fn main() {
    let my_tup = (500, 6.4, 1);

    let (x, y, z) = my_tup;

    println!("The value of y is: {y}");
}
```
- The above example first creates a tuple, `my_tup`, with three values.
- Next, the tuple `my_tup` is "destructured" - broken into three parts: `x`, `y`, and `z`.

- A tuple element can also be accessed directly by using a period (`.`), followed by the index of the value
 you want to access. For example:
 ```rust
 fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
 ```
 - This program creates the tuple x and then accesses each element of the tuple using their respective indices. As with most programming languages, the first index in a tuple is `0`.

 - The tuple without any values has a special name, _unit_. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

 #### The Array Type
 - Another way to have a collection of multiple values is with an _array_. Unlike a _tuple_, every element of an array must have the same type. Unlike arrays in some other languages, **arrays in Rust have a fixed length**.
 - We write the values in an array as a comma-separated list inside square brackets:
 ```rust
 fn main() {
    let a = [1, 2, 3, 4, 5];
 }
 ```
 - _Arrays_ are useful when you want your data allocated on the stack rather than the heap (more in Chapter 4) or when you want to ensure you always have a fixed number of elements.
 - An _array_ is **not** as flexible as a _vector_ type.
 - A _vector_ is a similar _collection_ type provided by the standard library that **is allowed to grow or shrink in size. If you’re unsure whether to use an _array_ or a _vector_, chances are you should use a _vector_**.
 - _Arrays_ are more useful **when you know the number of elements will not need to change**. For example, an array of months:

```rust
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
``` 

- You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
```rust
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

- Here, `i32` is the type of each element. After the semicolon, the number `5` indicates the array contains five elements.
- You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:
```rust
let a = [3; 5];
```
- The array named `a` will contain `5` _elements_ that will all be set to the _value_ `3` initially.
- This is the same as writing `let a = [3, 3, 3, 3, 3]`; but in a more concise way.

##### Accessing Array Elements
- An array is a single chunk of memory of a known, fixed size that can be allocated on the stack.
- You can access elements of an array by using indexing, like this:
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```
- In this example, the variable named `first` will get the value `1` because that is the value at index `[0]` in the array. The variable named `second` will get the value `2` from index `[1]` in the array.

##### Invalid Array Element Access
- Let’s see what happens if you try to access an element of an array that is past the end of the array. Say you run this code, similar to the guessing game in Chapter 2, to get an array index from the user:
```rust
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
```

- This code compiles successfully. If you run this code using cargo run and enter `0`, `1`, `2`, `3`, or `4`, the program will print out the corresponding value at that index in the array. If you instead enter a number past the end of the array, such as `10`, you’ll see output like this:

```rust
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- The program resulted in a _runtime_ error at the point of using an invalid value in the indexing operation. The program exited with an error message and didn’t execute the final `println!` statement.
- When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime, especially in this case, because the compiler can’t possibly know what value a user will enter when they run the code later.
- This is an example of Rust’s memory safety principles in action. In many low-level languages, this kind of check is not done, and when you provide an incorrect index, invalid memory can be accessed. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.