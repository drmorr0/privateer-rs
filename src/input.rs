use anyhow;
use std::fmt;
use std::io::{self, Write};

pub fn get_response<T: ToString + Copy>(prompt: &str, choices: &Vec<T>) -> anyhow::Result<T> {
    loop {
        let stringify = choices
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        print!("{} [{}] ", prompt, stringify);
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        response = response.to_lowercase();

        match choices
            .iter()
            .position(|c| c.to_string().starts_with(response.trim()))
        {
            Some(i) => return Ok(choices[i]),
            None => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            }
        }
    }
}

pub fn get_response_choices<T: fmt::Display>(choices: &Vec<T>) -> anyhow::Result<usize> {
    loop {
        let prompt = choices
            .iter()
            .enumerate()
            .map(|(i, x)| format!("[{}] {}", i + 1, x))
            .collect::<Vec<String>>()
            .join("; ");
        print!("{}: ", prompt);
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;

        match response.trim().parse::<usize>() {
            Ok(i) => return Ok(i - 1),
            Err(_) => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            }
        }
    }
}
