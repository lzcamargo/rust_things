fn main() {
    
    // Immutable and mutable variables ==========================================
    println!("Hello, world!");
    let _b: i32 = 3;
    println!("'immutable' The value of b is {} ", _b);

    let mut _a: i32 = 5;
    println!("The value of a is {} ", _a);
    _a = 10;
    println!("'mutable' The value of a now is {} ", _a);

    // Cosntant variable ======================================================

    let x = 5;
    const Y: i64 = 10;

     println!("'mutable' variable again, value x is {} ", x);
     println!("'constant' variable, value  is Y {} ", Y);
     println!("'constant' Global variable, value of PI is {} ", PI);
     println!("'constant' Global variable, value of PI_2 is {} ", PI_SQUARE);


   // Shadowing +++++++++++++++++++++++++++++++++++++++++++++++++++++++++
   // Shadowin is not the same as marking a variable as mutable
   let s = 5;
   let s = s + 1;
   println!("The value of s is: {s}");
   
   {
    let s = s * 2;
    println!("The value of s in the inner scope is: {s}");
   }
}
const PI: f64 = 3.141592;
const PI_SQUARE: f64 = PI * PI; 
