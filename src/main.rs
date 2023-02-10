use std::io::{stdin, stdout, Write};

fn main() {
    print!("Input the Message You Would Like to Encrypt: ");
    let _ = stdout().flush();

    let mut message = String::new();

    stdin()
        .read_line(&mut message)
        .expect("Could Not Read Input");

    let alphabet = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' '];
    let mut result: Vec<char> = vec![];

    message.truncate(message.len() - 2);
    let chars = message.chars();
    for (count, mut char) in chars.enumerate() {
        let mut index = alphabet.iter().position(|&v| v == char).unwrap_or(25 /* Space Character in Vector */);
        println!("IDX: {:?} WRD: {:?}", index, char);

        match index {
            25 => { index = index - 1 }
            26 => { index = 25 }
            _ => {}
        }

        char = alphabet[index + 1];

        result.insert(count, char)
    }
    let message: String = result.iter().collect();
    println!("Ciphered Message: {:?}", message)
}
