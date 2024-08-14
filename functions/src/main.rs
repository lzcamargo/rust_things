//snake case hello_world
//kebab case hello-world

//main function

fn main() {
  hello_world();
  tell_height(169);
  person_data("Luiz", 54, 169.2);
  
  // expressions can be code blocks
  let _x: i32 = {
    let price: i32 = 5;
    let qty: i32 = 10;
    price * qty
  };
  println!("Result is {} ", _x);

  //add 4 and 6
  let y: i32 = add(4,6);
  println!("The valeu of y is :{}", y);
  println!("O the value returned from add function is {} ", add(5,5));

  // calling the BMI function
  let weight: f64 = 80.1;
  let height: f64 = 1.69;
  let bmi = calculate_bmi(weight, height);
  println!("Your BMI is {:.2}", bmi );

  //........... Ownership propertis ..............//

  let str1 = String::from("RUSTCEAN");
  let len_str = calculate_length(&str1);
  println!("Length of '{}' is {}. ", str1, len_str);

  // print empty value
  let str2 = str1;
  println!("Length of '{}' is", str2);  

  //calling reference function
  reference_example();

  //calling mutable references function
  mutable_referece();

  // implementation struct bank

  let mut account: BankAccount = BankAccount {
    owner: "Alice ".to_string(),
    balance: 150.55,
  };
  // Immutable borrow to check the bank
  account.check_balance();

 //mutable borrow to withdraw money 
 account.withdraw(45.5);

  account.check_balance();


}

//invalid out of context (str1)
//println!("Length of {} is", &str1);  


fn hello_world() {
  println!("Hello Rust ");
}

fn tell_height(height: u32){
  println!("My height is {} cm.", height)
}

fn person_data(name: &str, age: u32, height: f32) {
  println!("My name is {} , I am {} years old, and my height is {} cm.", 
  name, age, height);
}

//expressions and statements
//expressions: anything that returns a value.
//statment: anything that does not return a value.
//Expression: 5, true or false, add(3,4) 

//functions returning values

fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn calculate_bmi(wight_kg: f64, height_m: f64) -> f64{
  wight_kg / (height_m * height_m)
}

//Ownership role
// a) Each value in Rust has a variable that's its owner. 
// b) There can be only owner at a time
// c) When the owner goes out of scope, the value will dropped 

fn calculate_length(st: &String) -> usize {
  st.len()

}

// references and borrowing
// safety and performance
// Understanding References
// References: Enable to borrow values without taking ownership
// Mutable and Immutable references
// Create References by add "&"

// Immutable Reference
fn reference_example() {
  let _x: i32 = 10;
  let _y: &i32 = &_x;

  println!("Value of _x is {} ", _x);
  println!("Value of _y is {} ", _y);
}

// mutable reference
fn mutable_referece() {
  let mut _x: i32 = 5;
  let _r: &mut i32 = &mut _x;

  *_r += 1;
  *_r -= 3; 

  println!("Value of _x is {} in mutable", _x) 
  //println!("Value of _r is {} in mutable ", _r);
}

struct BankAccount {
  owner: String,
  balance: f64,
}

impl BankAccount{
  fn withdraw(&mut self, amount: f64) {
    println!("Withdrawing {} from account owned by {} ", amount, self.owner);
    self.balance -= amount;
  }
  fn check_balance(&self){
    println!("Account owned by {} has a balance of {} ", self.owner, self.balance);
  } 
} 


