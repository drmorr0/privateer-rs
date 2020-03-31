mod components;
mod repl;
mod ship;
mod state_machine;
mod template;
mod world;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = state_machine::Context::new();
    ctx.run(Box::new(state_machine::EntryState {}))?;
    Ok(())

    // println!(
    //     "\nGreetings!  You are the captain of {}, a brand new {}!",
    //     ship.name,
    //     ship.hull().name
    // );
    // println!("It currently won't go very far since it's missing an engine.");
    // let choice = repl::get_response("Would you like to add an engine?", vec!["yes", "no"])?;
    // if choice == "no" {
    //     println!("You spend the rest of your days wistfully staring at your ship, wondering what might have been.");
    //     return Ok(());
    // }

    // println!("These are the engines currently available:\n");
    // for (i, engine_template) in engine_templates.iter().enumerate() {
    //     print!("[{}] {}\n", i, engine_template);
    // }
    // let choice = repl::get_response(
    //     "Which would you like to install?",
    //     (0..engine_templates.len()).collect::<Vec<usize>>(),
    // )?;

    // let mut score = 0;
    // match ship.add_component(ComponentType::Engine(engine_templates[choice].clone()), "Fuselage") {
    //     Ok(_) => {
    //         println!("You add an engine to your ship and take to the stars.  Game over.\n");
    //         score = 1000000000;
    //     }
    //     Err(_) => {
    //         println!("You attempt to force the engine into your ship, but it won't fit.");
    //         println!("You try harder, and the engine, your ship, and you, explode.  Game over.\n");
    //     }
    // }

    // println!("Your ship at the end of the game:\n");
    // println!("{}", ship);
    // println!("Final Score: {}", score);

    // Ok(())
}
