use crate::util::enumiter;
use std::{
    fmt,
    io::{
        self,
        Write,
    },
};

pub fn get_response_yn(prompt: &str) -> bool {
    get_response_inline(prompt, &mut vec!["yes", "no"]) == "yes"
}

pub fn get_response_inline<T: ToString>(prompt: &str, choices: &mut Vec<T>) -> T {
    loop {
        let responses = choices.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");
        print!("{} [{}] ", prompt, responses);
        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        response = response.trim().to_lowercase();

        match choices.iter().position(|c| c.to_string().starts_with(&response)) {
            Some(i) => return choices.swap_remove(i),
            None => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            },
        }
    }
}

pub fn get_response_choices<'a, T: fmt::Display, U>(prompt: &str, choices: &mut Vec<(T, U)>) -> U {
    get_response_choices_helper(prompt, choices, None)
}

pub fn get_response_choices_or_back<'a, T: fmt::Display, U>(
    prompt: &str,
    choices: &mut Vec<(T, U)>,
    back_choice: U,
) -> U {
    get_response_choices_helper(prompt, choices, Some(back_choice))
}

fn get_response_choices_helper<'a, T: fmt::Display, U>(
    prompt: &str,
    choices: &mut Vec<(T, U)>,
    back_choice: Option<U>,
) -> U {
    println!("{}", prompt);
    loop {
        let mut choice_str = enumiter(choices)
            .map(|(i, x)| format!("  [{}] {}", i + 1, x.0))
            .collect::<Vec<String>>()
            .join("\n");
        if back_choice.is_some() {
            choice_str += &format!("\n  [{}] Back\n", choices.len() + 1)
        }
        print!("{}\n> ", choice_str);

        io::stdout().flush().unwrap();

        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();

        response = response.trim().to_lowercase();
        if &response == "quit" {
            std::process::exit(0);
        }

        match response.parse::<usize>() {
            // We want to return the actual object that was chosen which means we have to take it
            // out of the vector and take ownership of it; this, in turn, requires the vector to
            // be mutable.
            Ok(i) if i <= choices.len() => return choices.swap_remove(i - 1).1,
            Ok(i) if i == choices.len() + 1 && back_choice.is_some() => return back_choice.unwrap(),
            _ => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            },
        }
    }
}
