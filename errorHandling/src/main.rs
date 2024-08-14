// error handling

  // approach 01
  //enum Option<T> { // define the generic Option type
 //   Some(T), // represents a value
 //   None, // represents no value
 // }
/*
  fn divideOption(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
      None
    } else {
      Some(numerator / denominator)
    }
  }
*/
  fn divideResult(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
      Err("Cannot divide the number by zero".to_string())
    } else {
      Ok(numerator / denominator)
    }

  }    




  // approach 02
  
 // enum Result<T, E>{ // define the generic Result type
 //   Ok(T), // represents a value
 //   Err(E), // represents an error
 //}
fn main() {
 /* let result  = divideOption(10.0, 3.0);
  match result {
    Some(n) => println!("The result is {n}"),
    None => println!("The result is none, cannot divide the number by zero"),
  }
   */
  match divideResult(100.23, 73.98){
    Ok(result) => println!("The result is {}", result),
    Err(err) => println!("The result is {}", err),
  }
  
}
