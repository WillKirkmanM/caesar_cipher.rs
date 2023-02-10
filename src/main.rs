use std::io::{stdin, stdout, Write};

fn main() {
    print!("Input the Message You Would Like to Encrypt: ");
    let _ = stdout().flush();

    let mut message = String::new();

    stdin()
        .read_line(&mut message)
        .expect("Could Not Read Input");

    let alphabet = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];

    message.truncate(message.len() - 2);
    let chars = message.chars();
    for char in chars {
        println!("{:?}", char)
    }
}
