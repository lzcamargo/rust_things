use std::io;
fn main() {
  let user_id = 12345;
  get_username(user_id);
}

  fn get_username(user_id: i32) {
    println!("Informe o nome :");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Leitura falhou");
    println!("Código do usuário: {}, Nome: {}", user_id, input);
    
  }  

  
