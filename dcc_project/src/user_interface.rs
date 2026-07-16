use std::io::{self, Error};

pub fn display_on_screen(text: &str) {
    println!("{}", text);
}

pub fn ask_user(question: &str) -> Result<String, Error> {
    println!();
    println!();
    let mut input = String::new();
    println!("{}  : ", question);
    match io::stdin().read_line(&mut input) {
        Ok(_size) => Ok(String::from(input.trim())),
        Err(error) => Err(error),
    }
}
