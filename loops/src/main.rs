fn main() {
  //loop {
    // println!("Hello, world!");
  // } 
  
  // Loops++++++++++++++++++++++++++++
  let mut counter = 0;

  let result = loop {
    counter += 1;
    if counter == 10 {
      break counter * 2;
    }
  };
  println! ("The result is {result}");

let mut count = 0;
'counting_up: loop {
  println!("count: {count}");
  let mut remaining = 10;
  loop {
    println!("Remaining: {remaining}");
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


// loops with While..........................................

let mut number = 3;
while number != 0 {
  println!("Number: {number}");
  number -= 1;
  break;
}
println!("Hello,,,,,,,");

// Loops with for....................................................

let a = [1, 2, 3, 4, 5, 6];
for element in a {
  println!("Element: {element}");
}

let b = ["a","b","c","d"];
for letter in b {
  println!("letter: {letter}");
}


}
