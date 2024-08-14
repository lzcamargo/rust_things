// collections types 
// vectors - UTF8 - Hasmaps

fn main() {
  let _v:Vec<i32> = Vec::new();
  // Macro to create a vector of numbers
  let _v: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,10];
  let mut _v:Vec<i32> = Vec::new();
  _v.push(1);
  _v.push(2);
  _v.push(3);

  println!("vector {:?}", _v);
  
  // catch the element of the vector
  let third: &i32 = &_v[2]; // direct index 
  println!("The third element is {}", third);

  let third: Option<&i32> = _v.get(7);
  match third {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element"),
  }


  // ---------------------------- UTF-8 --------------------------------

  // 1
  let s: String = "Whatever".to_string();
  // 2
  let s: String = String::from("whatever");
  // mutate the variable to push it
  let mut s: String = String::from("foo");
  s.push_str("bar");
  s.push('!');

  println!("the value of the s is {}", s);

// to combine the string with the operator +

let s1: String = String::from("Hello");
let s2: String = String::from("World");
let s3: String = s1 + &s2;

println!("s3 is {}", s3);

let text1: String = String::from("Texto");
let text2: String = String::from("xxxxxxxx");

let full_text: String = format!("{text1} {text2}");

println!("full_text is {}", full_text);




}
