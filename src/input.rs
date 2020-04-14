use std::{
    fmt,
    io::{
        self,
        Write,
    },
};

pub fn get_response<T: ToString + Copy>(prompt: &str, choices: &Vec<T>) -> anyhow::Result<T> {
    loop {
        let stringify = choices.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        print!("{} [{}] ", prompt, stringify);
        io::stdout().flush()?;

        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        response = response.to_lowercase();

        match choices.iter().position(|c| c.to_string().starts_with(response.trim())) {
            Some(i) => return Ok(choices[i]),
            None => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            },
        }
    }
}

pub fn get_response_choices<'a, T: fmt::Display, U>(prompt: &str, choices: &mut Vec<(T, U)>) -> U {
    println!("{}", prompt);
    loop {
        let choice_str = choices
            .iter()
            .enumerate()
            .map(|(i, x)| format!("  [{}] {}", i + 1, x.0))
            .collect::<Vec<String>>()
            .join("\n");
        print!("{}\n> ", choice_str);
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        match response.trim().parse::<usize>() {
            // We want to return the actual object that was chosen which means we have to take it
            // out of the vector and take ownership of it; this, in turn, requires the vector to
            // be mutable.
            Ok(i) => return choices.swap_remove(i - 1).1,
            Err(_) => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            },
        }
    }
}
