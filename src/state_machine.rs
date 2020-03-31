use crate::repl;
use crate::world::World;
use std::io::{self, Write};
use anyhow;

type StatePointer = anyhow::Result<Option<Box<dyn State>>>;

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
        Ok(Some(Box::new(NewShipState { ship_id })))
    }
}

struct NewShipState {
    ship_id: usize,
}

impl State for NewShipState {
    fn enter(&self, world: &World) -> anyhow::Result<()> {
        let ship = &world.ships[self.ship_id];
        println!("Congratulations!  You are the proud owner of {}", ship.name);
        println!("It won't go far without an engine, though.");
        Ok(())
    }
    
    fn handle_input(&self, world: &mut World) -> StatePointer {
        let choice = repl::get_response("Would you like to add an engine?", vec!["yes", "no"])?;
        match choice {
            "yes" => Ok(Some(Box::new(ExitState { message: "You add an engine to your ship and take to the stars!".to_string()}))),
            "no" => Ok(Some(Box::new(ExitState { message: "You spend the rest of your days wistfully staring at your ship, wondering what might have been.".to_string() }))),
            _ => panic!("ohno"),
        }
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
        Ok(None)
    }
}

pub struct Context {
    current_state: Option<Box<dyn State>>,
    world: World,
}

impl Context {
    pub fn new() -> Context {
        Context {
            current_state: None,
            world: World::new(),
        }
    }

    pub fn run(&mut self, starting_state: Box<dyn State>) -> anyhow::Result<()> {
        self.current_state = Some(starting_state);

        loop {
            match &self.current_state {
                Some(s) => {
                    s.enter(&self.world)?;
                    self.current_state = s.handle_input(&mut self.world)?;
                }
                None => break,
            }
        }
        Ok(())
    }
}
