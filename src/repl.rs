use std::io::{self, Write};

pub fn get_response<T: ToString + Copy>(prompt: &str, choices: Vec<T>) -> Result<T, io::Error> {
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
