#[cfg(test)]
use crate::cipher::cipher_message;

#[test]
fn cipher_lowercase_text() {
  let message = String::from("hello world");
  let ciphered_text = cipher_message(message);
  
  assert_eq!(ciphered_text, "ifmmp xpsme")
}

#[test]
fn cipher_uppercase_text() {
  let message = String::from("HELLO WORLD");
  let ciphered_text = cipher_message(message);
  
  assert_eq!(ciphered_text, "IFMMP XPSME")
}

#[test]
fn cipher_symbols() {
  let message = String::from("!@#$%^&*()_+{}");
  let ciphered_text = cipher_message(message);
  
  assert_eq!(ciphered_text, "!@#$%^&*()_+{}")
}

#[test]
fn cipher_space() {
  let message = String::from("       ");
  let ciphered_text = cipher_message(message);
  
  assert_eq!(ciphered_text, "") // .trim() Trims All Whitespace
}

#[test]
fn cipher_mix() {
  let message = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz !@#$%^&*()_+{}");
  let ciphered_text = cipher_message(message);
  
  assert_eq!(ciphered_text, "BCDEFGHIJKLMNOPQRSTUVWXYZA bcdefghijklmnopqrstuvwxyza !@#$%^&*()_+{}")
}
