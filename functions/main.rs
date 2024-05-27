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

}

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


