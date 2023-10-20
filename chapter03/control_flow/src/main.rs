fn main() {
    // Test 1
    let number = 3;
  
    if number < 5 {
      println!("condition was true");
    } else {
      println!("condition was false");
    }

    // Test 2
    let number2 = 7;

    if number2 < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Test 3
    // let number3 = 3;

    // if number3 {
    //     println!("number was three");
    // }

    // Test 4
    let number4 = 3;

    if number4 != 0 {
      println!("number was something other than 0.");
    }

    // Test 5: Handling Multiple Conditions with else if
    let number5 = 6;

    if number5 % 4 == 0 {
      println!("number is divisible by 4.");
    } else if number5 % 3 == 0 {
      println!("number is divisible by 3.");
    } else if number5 % 2 == 0 {
      println!("number is divisible by 2.");
    } else {
      println!("number is not divisible by 4, 3, or 2.");
    }
}