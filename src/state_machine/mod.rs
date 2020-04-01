pub mod ship_builder;
use crate::repl;
use crate::world::World;
use std::io::{self, Write};
use std::error::Error as StdError;
use std::fmt;
use anyhow::anyhow;

type StatePointer = anyhow::Result<Box<dyn State>>;

#[derive(Debug)]
struct StopIteration { }

impl StdError for StopIteration { }

impl fmt::Display for StopIteration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "StopIteration")
    }
}

pub trait State {
    fn enter(&self, world: &World) -> anyhow::Result<()>;
    fn handle_input(&self, world: &mut World) -> StatePointer;
}

pub struct EntryState {}

impl State for EntryState {
    fn enter(&self, _: &World) -> anyhow::Result<()> {
        println!("Greetings, Captain!  You have come into possession of a new ship!");
        print!("What would you like to name it? ");
        io::stdout().flush()?;
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> StatePointer {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error reading input");
        let ship_id = world.mk_ship(response, world.templateStore.hull_templates[0].clone());
        Ok(Box::new(BuilderRootState { ship_id }))
    }
}

struct ExitState {
    message: String,
}

impl State for ExitState {
    fn enter(&self, _: &World) -> anyhow::Result<()> {
        println!("{}", self.message);
        println!("Game over, man!  Game over!");
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> StatePointer {
        Err(anyhow!(StopIteration { }))
    }
}

pub fn run(world: &mut World, starting_state: Box<dyn State>) -> anyhow::Result<()> {
    let mut current_state = starting_state;

    loop {
        current_state.enter(&*world)?;
        match current_state.handle_input(world) {
            Ok(s) => current_state = s,
            Err(e) => match e {
                StopIteration => break,
                _ => return Err(e)
            }
        };
    }
    Ok(())
}
