#[cfg(test)]
use crate::decipher::decipher_message;

#[test]
fn decipher_lowercase_text() {
  let message = String::from("ifmmp xpsme");
  let deciphered_text = decipher_message(message);
  
  assert_eq!(deciphered_text, "hello world")
}

#[test]
fn decipher_uppercase_text() {
  let message = String::from("IFMMP XPSME");
  let deciphered_text = decipher_message(message);
  
  assert_eq!(deciphered_text, "HELLO WORLD")
}

#[test]
fn decipher_symbols() {
  let message = String::from("!@#$%^&*()_+{}");
  let deciphered_text = decipher_message(message);
  
  assert_eq!(deciphered_text, "!@#$%^&*()_+{}")
}

#[test]
fn decipher_space() {
  let message = String::from("       ");
  let deciphered_text = decipher_message(message);
  
  assert_eq!(deciphered_text, "") // .trim() Trims All Whitespace
}

#[test]
fn decipher_mix() {
  let message = String::from("BCDEFGHIJKLMNOPQRSTUVWXYZA bcdefghijklmnopqrstuvwxyza !@#$%^&*()_+{}");
  let deciphered_text = decipher_message(message);
  
  assert_eq!(deciphered_text, "ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz !@#$%^&*()_+{}")
}
