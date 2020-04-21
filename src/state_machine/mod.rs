pub mod ship;
pub mod ship_builder;
use crate::{
    template::TemplateStore,
    world::World,
};
use anyhow::Result as AnyResult;
use std::io::{
    self,
    Write,
};

type StatePointer = Box<dyn State>;

pub enum ContextAction {
    Pushdown(StatePointer),
    Replace(StatePointer),
    Bounce,
}

pub trait State {
    fn enter(&self, world: &World) -> AnyResult<()>;
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction>;
}

pub struct EntryState {}

impl State for EntryState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        println!("Greetings, Captain!  You have come into possession of a new ship!");
        print!("What would you like to name it? ");
        io::stdout().flush()?;
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let mut response = String::new();
        io::stdin().read_line(&mut response).expect("Error reading input");
        let ship_id = world.mk_ship(response.trim(), TemplateStore::hull(0).unwrap().clone());
        Ok(ContextAction::Replace(Box::new(ship_builder::BuilderRootState {
            ship_id,
            shop_id: 0,
        })))
    }
}

struct ExitState {
    message: String,
}

impl State for ExitState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        println!("{}", self.message);
        println!("Game over, man!  Game over!");
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        Ok(ContextAction::Bounce)
    }
}

pub struct Context {
    stack: Vec<Box<dyn State>>,
    world: Box<World>,
}

impl<'ctx> Context {
    pub fn new(starting_state: Box<dyn State>) -> Context {
        Context {
            stack: vec![starting_state],
            world: Box::new(World::new()),
        }
    }

    pub fn run(&mut self) -> AnyResult<()> {
        loop {
            let current_state = self.stack.last();
            match current_state {
                Some(s) => {
                    s.enter(&self.world)?;
                    match s.handle_input(&mut self.world).unwrap() {
                        ContextAction::Pushdown(s) => self.stack.push(s),
                        ContextAction::Replace(s) => {
                            self.stack.pop();
                            self.stack.push(s);
                        },
                        ContextAction::Bounce => {
                            self.stack.pop();
                        },
                    }
                },
                None => break,
            }
        }
        Ok(())
    }
}
