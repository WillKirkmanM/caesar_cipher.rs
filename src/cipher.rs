use std::io::{stdin, stdout, Write};

pub fn cipher() {
  print!("Input the Message You Would Like to Encrypt: ");
  stdout().flush().unwrap();

  let mut message = String::new();

  stdin()
      .read_line(&mut message)
      .expect("Could Not Read Input");

  let alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' '];
  let mut result: Vec<char> = vec![];

  let chars = message.trim().chars();
  let mut uppercase_vec: Vec<usize> = vec![];
  for (count, mut char) in chars.enumerate() {
    if char.is_ascii_uppercase() {
      uppercase_vec.push(count);
      char = char.to_ascii_lowercase();
    }
    let mut index = alphabet.iter().position(|&v| v == char).unwrap_or(26 /* Space Character in Vector */);

    match index {
      25 => index = 0,
      26 => index = 26,
      _ => index = index + 1
    }
    char = alphabet[index];
    result.insert(count, char)
  }

  for index in uppercase_vec {
    for char in result.to_owned() {
      if char == result[index] { 
        result[index] = char.to_ascii_uppercase() 
      } 
    }
  }

  let message: String = result.iter().collect();
  println!("Ciphered Message: {:?}", message);
}