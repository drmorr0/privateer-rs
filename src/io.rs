use crate::util::enumiter;
use ron::de::from_reader;
use serde::de::DeserializeOwned;
use std::{
    fmt,
    fs::File,
    io,
    io::Write,
};

pub fn prompt(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().to_string()
}

pub fn match_response_yn(response: &str) -> Option<bool> {
    match response {
        "yes" => Some(true),
        "no" => Some(false),
        _ => None,
    }
}

pub fn prompt_choices<'a, T: fmt::Display, U: Clone>(prompt: &str, choices: &Vec<(T, U)>) {
    println!("{}", prompt);
    let choice_str = enumiter(choices)
        .map(|(i, x)| format!("  [{}] {}", i + 1, x.0))
        .collect::<Vec<String>>()
        .join("\n");
    println!("{} ", choice_str);
}

pub fn match_choice<T, U: Clone>(response: &str, choices: &Vec<(T, U)>) -> Option<U> {
    match response.parse::<usize>() {
        Ok(i) if i <= choices.len() => return Some(choices[i - 1].1.clone()),
        _ => None,
    }
}

pub fn match_command_choice<'a, U>(command: &str, tokens: &[String], choices: &'a Vec<U>) -> Option<&'a U> {
    match (tokens[0].as_str(), tokens[1].parse::<usize>()) {
        (c, Ok(i)) if c == command && i <= choices.len() => Some(&choices[i - 1]),
        _ => None,
    }
}

pub fn read_data_file<T: DeserializeOwned>(filename: &str) -> T {
    let f = File::open(filename).unwrap();
    let reader = io::BufReader::new(f);
    from_reader(reader).unwrap()
}
