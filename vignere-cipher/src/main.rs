use std::io;

use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;

fn main() {

    println!("Please choose from the following menu.");
    let selected = select_screen();
    let mut key = String::new();
    
    match selected {
        Ok(val) => {
            if val == "Encode" {
                let mut message = String::new();
                println!("Please input your message.");
                io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");
                
                
                println!("Please input your secret encoding key.");
                io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");

                let res = encode_vignere(message, key);
                println!("your ciphertext ser: {}", res);
            } else {
                let mut ciphertext = String::new();
                println!("Please input your ciphertext.");
                io::stdin()
                .read_line(&mut ciphertext)
                .expect("Failed to read line");
                
                
                println!("Please input the secret decoding key.");
                io::stdin()
                .read_line(&mut key)
                .expect("Failed to read line");
                
                let res = decode_vignere(ciphertext, key);
                println!("your message ser: {}", res);
            }
        },
        Err(err) => {
            // Do something with the error if you want
            println!("Error: {}", err);
        }
    }

    

    
}

const ALPHABET: String = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string();

fn char_to_pos(input: char) -> usize {
    ALPHABET.chars().position(|c| input == c).unwrap()
}


fn encode_vignere(m: String, k: String) -> String {
    let mut result: String = "".to_string();
    for message_char in m.chars() {
        for key_char in k.chars() {
            let new_char_index = char_to_pos(message_char) + char_to_pos(key_char);
            let new_char = ALPHABET.chars().nth(new_char_index).unwrap();
            result.push(new_char);
        }
    }
    result
}

fn decode_vignere(c: String, k: String) -> String {
    let mut result: String = "".to_string();

    result
}

fn select_screen() -> std::io::Result<(String)> {
    let items = vec!["Encode", "Decode"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => Ok(items[index].to_string()),
        None => Ok("".to_string())
    }

}