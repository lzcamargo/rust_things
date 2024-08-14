// if expression [if ... ] [Else expression]
//#![allow(warnings)]
fn main() {
  let age: u16 = 18;
  if age >= 18 {
    println! (" you can to get license-driver")

  } else {
    println!("You can't to get license-driver");
  }

  // multiple conditions with if else
  let number = 11;
  if number % 4 == 0 {
    println!("number is divisible by 4");
  }
  else if number % 3 == 0 {
    println!("number is divisible by 3");
  }
  else if number % 2 == 0 {
    println!("number is divisible by 2");
  }
  else {
    println!("number is not divisible by 2,3 or 4");
  }

  // using if in a let statement

  let condition = true;
  let number = if condition {5} else {6};
  println!("Number: {number}");






}