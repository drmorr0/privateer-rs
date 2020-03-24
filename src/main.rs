mod components;
mod ship;
mod template;
use std::error::Error;
use std::io;
use crate::components::ComponentType;

fn main() -> Result<(), Box<dyn Error>> {
    let hull_templates: Vec<components::HullData> =
        template::read_template_file("./data/hulls.ron".to_string());
    let engine_templates: Vec<components::EngineData> =
        template::read_template_file("./data/engines.ron".to_string());
    let mut ship = ship::Ship::new("The Flying Dutchman".to_string(), hull_templates[0].clone());

    let stdin = io::stdin();
    println!(
        "\nGreetings!  You are the captain of {}, a brand new {}!",
        ship.name,
        ship.hull().name
    );
    println!("It currently won't go very far since it's missing an engine.");
    println!("Would you like to add an engine? (y/n)\n");

    loop {
        let mut response = String::new();
        stdin.read_line(&mut response)?;
        match response.trim() {
            "y" => break,
            "n" => {
                println!("You spend the rest of your life staring wistfully at your ship, wondering what might have been.");
                return Ok(());
            },
            _ => {
                println!("Sorry, I didn't understand.  Please try again.");
                continue;
            },
        }
    }

    println!("These are the engines currently available; which one do you want?");
    for (i, engine_template) in engine_templates.iter().enumerate() {
        print!("[{}] {}\n", i, engine_template);
    }

    loop {
        let mut response = String::new();
        stdin.read_line(&mut response)?;
        match response.trim().parse::<usize>() {
            Ok(n) if n < engine_templates.len() => {
                let res = ship.add_component(ComponentType::Engine(engine_templates[n].clone()), "Fuselage");
                match res {
                    Err(s) => println!("Sorry, I couldn't add that engine: {}", s),
                    _ => break,
                }
            },
            _ => {
                println!("Invalid choice.  Please try again.");
                continue;
            },
        }
    }

    println!("You add an engine to your ship and take to the stars.  Game over.\n");
    println!("Your ship at the end of the game:\n");
    println!("{}", ship);
    println!("Final Score: 100000000");

    Ok(())
}
