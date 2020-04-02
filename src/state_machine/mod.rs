pub mod ship_builder;
use crate::world::World;
use std::io::{self, Write};

type StatePointer = Box<dyn State>;

pub enum ContextAction {
    Pushdown(StatePointer),
    Replace(StatePointer),
    Bounce,
    Err(anyhow::Error),
}

pub trait State {
    fn enter(&self, world: &World) -> anyhow::Result<()>;
    fn handle_input(&self, world: &mut World) -> ContextAction;
}

pub struct EntryState {}

impl State for EntryState {
    fn enter(&self, _: &World) -> anyhow::Result<()> {
        println!("Greetings, Captain!  You have come into possession of a new ship!");
        print!("What would you like to name it? ");
        io::stdout().flush()?;
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> ContextAction {
        let mut response = String::new();
        io::stdin()
            .read_line(&mut response)
            .expect("Error reading input");
        let ship_id = world.mk_ship(response, world.template_store.hull_templates[0].clone());
        ContextAction::Replace(Box::new(ship_builder::BuilderRootState { ship_id }))
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

    fn handle_input(&self, _: &mut World) -> ContextAction {
        ContextAction::Bounce
    }
}

pub struct Context {
    stack: Vec<Box<dyn State>>,
    world: World,
}

impl Context {
    pub fn new(starting_state: Box<dyn State>) -> Context {
        Context {
            stack: vec![starting_state],
            world: World::new(),
        }
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let current_state = self.stack.last();
            match current_state {
                Some(s) => {
                    s.enter(&self.world)?;
                    match s.handle_input(&mut self.world) {
                        ContextAction::Pushdown(s) => self.stack.push(s),
                        ContextAction::Replace(s) => {
                            self.stack.pop();
                            self.stack.push(s);
                        }
                        ContextAction::Bounce => {
                            self.stack.pop();
                        }
                        ContextAction::Err(e) => {
                            return Err(e);
                        }
                    }
                }
                None => break,
            }
        }
        Ok(())
    }
}
