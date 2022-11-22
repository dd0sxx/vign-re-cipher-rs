use std::io;

use dialoguer::{
    Select,
    theme::ColorfulTheme
};
use console::Term;


fn main() {

    println!("Please choose from the following menu.");
    let selected = select_screen();
    let mut message = String::new();
    let mut key = String::new();

    match selected {
        Ok(val) => {
            if val == "Encode" {
                println!("Please input your message.");
                io::stdin()
                    .read_line(&mut message)
                    .expect("Failed to read line");
                
    
                println!("Please input your secret encoding key.");
                io::stdin()
                    .read_line(&mut key)
                    .expect("Failed to read line");
            } else {
                println!("Please input your ciphertext.");
                io::stdin()
                    .read_line(&mut message)
                    .expect("Failed to read line");
                
    
                println!("Please input the secret decoding key.");
                io::stdin()
                    .read_line(&mut key)
                    .expect("Failed to read line");
            }
        },
        Err(err) => {
            // Do something with the error if you want
            println!("Invalid selection...");
        }
    }

    

    
}

fn encode_vignere(m: String, k: String) {

}

fn decode_vignere(c: String, k: String) {

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