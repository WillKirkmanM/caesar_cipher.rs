use std::io::{stdin, stdout, Write};

use super::cipher::cipher;
use super::decipher::decipher;

pub fn init() {
  println!("1: Encode Message");
  println!("2: Decode Message");

  print!("Enter your Choice: ");
  stdout().flush().unwrap();

  let mut choice = String::new();
  stdin()
    .read_line(&mut choice)
    .expect("Enter a Valid Message");

  match choice.as_str().trim() {
    "1" => cipher(),
    "2" => decipher(),
    _ => { 
      print!("\x1B[2J\x1B[1;1H"); // <- Clears Terminal
      init()
    }
  }


}