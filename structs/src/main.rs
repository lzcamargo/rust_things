#![allow(warnings)] 
fn main() {
  let _rect: (i32, i32) = (200,500);

  // Structs
  struct Book {
    title: String,
    author: String,
    pages: u32,
    available: bool,
  }

  struct User {
    active: bool, 
    username: String,
    email: String,
    sign_in_count: u64,
  }

  let mut user1: User = User {
    active: true,
    username: String::from("Luiz,.,,,,."),
    email: String::from("luiz@.com"),
    sign_in_count:1,
  };
  user1.email = String::from("anoter@com");
  println!("User {} {}", user1.username, user1.email );

  // return a struct from a function

  fn build_user(email: String, username: String) -> User{
    User {
      active: true,
      email,
      username,
      sign_in_count:1,
    }
  }

  // create instances from other instances
  let user2: User = User{
    email: String::from("Another@xxx.com"),
    ..user1,
  };

  // tuple Structs

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);

  let black: Color = Color(0,0,0);
  let white: Color = Color(255,255,255);

  // Unit-like structs
  struct AlwaysEqual;
  let subject: AlwaysEqual = AlwaysEqual;





}
