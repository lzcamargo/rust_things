fn main() {
  println!("Hello, world!");
  //integer i and u variables
  let y: i32 = 10;
  let x: u64 = 10; 
  println!("signed variable {}", y);
  println!("unsigned variable {}", x);

  //float
  let pi: f64 = 3.14565656565656;  
  println!("show the values of pi {}", pi);
  //bolean
  let bol: bool = true;
  println!("It's raining ? {}", bol);
  //char
  let letter: char = 'a';
  println!("is a letter {}", letter); 

  //arrays
  let numbers: [i32; 5] = [1,2,3,4,5];
  println!("printer array: {:?} ", numbers);

  let fruits: [&str; 3] = ["banana","orange", "apple"];
  println!("print string array {:?} ", fruits);
  println!("print 2st array element {} ", fruits[1]);
  println!("print 1st array element {} ", fruits[0]);

  //tuples
  let human: (String, i32, bool) = ("Joao".to_string() , 30, false);
  println!("print human {:?} ", human);
  let mix_tuple = ("Joao",23, true, [1,2,3,4]);
  println!("print mixed {:?} ", mix_tuple);

  //slices
  let number_slices :&[i32] = &[1,2,3,4,5];
  println!("Number slices {:?}", number_slices);

  let animal_slices :&[&str] = &["Lion", "Elefant", "Tiger"];
  println!("Animal slices {:?}", animal_slices);

  let book_slices :&[&String] = &[&"IT".to_string(), &"Geografy".to_string()];
  println!("Book slices {:?}", book_slices);

  //String vs String Slices {&str}
  let mut strin: String = String::from("Hell, ");
  strin.push_str("Yeah ");
  println!("Strin says: {}", strin);  

    //&str
  let string: String = String::from("Hello World ");
  let slice: &str = &string[0..5];
  println!("lice value: {}", slice); 











}
