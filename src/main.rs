mod components;
mod repl;
mod ship;
mod template;

use std::error::Error;
use crate::components::ComponentType;

fn main() -> Result<(), Box<dyn Error>> {
    let hull_templates: Vec<components::HullData> =
        template::read_template_file("./data/hulls.ron".to_string());
    let engine_templates: Vec<components::EngineData> =
        template::read_template_file("./data/engines.ron".to_string());
    let mut ship = ship::Ship::new("The Flying Dutchman".to_string(), hull_templates[0].clone());

    println!(
        "\nGreetings!  You are the captain of {}, a brand new {}!",
        ship.name,
        ship.hull().name
    );
    println!("It currently won't go very far since it's missing an engine.");
    let choice = repl::get_response("Would you like to add an engine?", vec!["yes", "no"])?;
    if choice == "no" {
        println!("You spend the rest of your days wistfully staring at your ship, wondering what might have been.");
        return Ok(());
    }

    println!("These are the engines currently available:\n");
    for (i, engine_template) in engine_templates.iter().enumerate() {
        print!("[{}] {}\n", i, engine_template);
    }
    let choice = repl::get_response(
        "Which would you like to install?", 
        (0..engine_templates.len()).collect::<Vec<usize>>(),
    )?;

    let mut score = 0;
    match ship.add_component(ComponentType::Engine(engine_templates[choice].clone()), "Fuselage") {
        Ok(_) => {
            println!("You add an engine to your ship and take to the stars.  Game over.\n");
            score = 1000000000;
        }
        Err(_) => {
            println!("You attempt to force the engine into your ship, but it won't fit.");
            println!("You try harder, and the engine, your ship, and you, explode.  Game over.\n");
        }
    }

    println!("Your ship at the end of the game:\n");
    println!("{}", ship);
    println!("Final Score: {}", score);

    Ok(())
}
