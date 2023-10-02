# Data Types
The [Chapter 3.2 Exercise](https://doc.rust-lang.org/book/ch03-02-data-types.html) from [The Rust Programming Language book](https://doc.rust-lang.org/book/).

## Additional Notes
### Scalar Data Types
A _scalar_ data type represents a single value. Rust has four primary scalar types:
 1. Integers
 1. Floating-point numbers
 1. Booleans
 1. Characters
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
```
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```