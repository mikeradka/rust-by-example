fn main() {
  // In general, the `{}` will be automatically replaced with any
  // arguments. These will be stringified.
  println!("Testing {}", 10);

  // Positional arguments can be used. Specifying an integer inside `{}`
  // determines which additional argument will be replaced. Arguments start
  // at 0 immediately after the format string.
  println!("{0}, this is {1}. {1}, this is {0}.", "Alice", "Bob");
  
  // As can named arguments.
  println!("{subject} {verb} {object}",
  object="the lazy dog",
  subject="the quick brown fox",
  verb="jumps over");

  // Different formatting can be invoked by specifying the format character
  // after a `:`.
  println!("Base 10:               {}",   69420); // 69420
  println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
  println!("Base 8 (octal):        {:o}", 69420); // 207454
  println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
  println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

  // You can right-justify text with a specified width. This will
  // output "    1". (Four white spaces and a "1", for a total width of 5.)
  println!("{number:>5}", number=1);

  // You can pad numbers with extra zeroes,
  println!("{number:0>5}", number=1); // 00001
  // and left-adjust by flipping the sign. This will output "10000".
  println!("{number:0<5}", number=1); // 10000

  // Rust even checks to make sure the correct number of arguments are used.
  println!("My name is {0}, {1} {0}", "Bond", "James");
  // FIXME ^ Add the missing argument: "James"
}