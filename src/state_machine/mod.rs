pub mod ship;
pub mod ship_builder;
mod util;
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

#[derive(Clone)] // These get cloned during the input::match_choice routine
pub enum ContextAction {
    Pushdown(StatePointer),
    Replace(StatePointer),
    Bounce,
    Clear,
    Retry,
}

#[derive(Eq, PartialEq)]
pub enum ResponseType {
    None,
    Raw,
    Tokenized,
}

pub trait State: util::BoxClone {
    fn enter(&self, world: &World) -> ResponseType;
    fn transition(&self, tokens: &Vec<String>, world: &mut World) -> Option<ContextAction>;
}

#[derive(Clone)]
pub struct EntryState {}

impl State for EntryState {
    fn enter(&self, _: &World) -> ResponseType {
        println!("Greetings, Captain!  You have come into possession of a new ship!");
        print!("What would you like to name it? ");
        io::stdout().flush().unwrap();
        ResponseType::Raw
    }

    fn transition(&self, input: &Vec<String>, world: &mut World) -> Option<ContextAction> {
        let ship_id = world.mk_ship(&input[0], TemplateStore::hull(0).unwrap().clone());
        Some(ContextAction::Replace(ship_builder::BuilderRootState::new(ship_id, 0)))
    }
}

#[derive(Clone)]
struct ExitState {
    message: String,
}

impl State for ExitState {
    fn enter(&self, _: &World) -> ResponseType {
        println!("{}", self.message);
        println!("Game over, man!  Game over!");
        ResponseType::None
    }

    fn transition(&self, _: &Vec<String>, _: &mut World) -> Option<ContextAction> {
        Some(ContextAction::Clear)
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
        while let Some(current_state) = self.stack.last() {
            let response_type = current_state.enter(&self.world);

            let action: ContextAction;
            loop {
                print!("> ");
                io::stdout().flush().unwrap();
                let mut response = String::new();
                let tokens = match response_type {
                    ResponseType::None => vec![response],
                    ResponseType::Raw => {
                        io::stdin().read_line(&mut response).unwrap();
                        vec![response.trim().to_string()]
                    },
                    ResponseType::Tokenized => {
                        io::stdin().read_line(&mut response).unwrap();
                        response
                            .trim()
                            .to_lowercase()
                            .split_ascii_whitespace()
                            .map(|s| s.to_string())
                            .collect()
                    },
                };

                if let Some(a) = self.process_global_command(&tokens) {
                    action = a;
                    break;
                } else if let Some(a) = current_state.transition(&tokens, &mut self.world) {
                    action = a;
                    break;
                } else {
                    println!("Sorry, I didn't understand.  Please try again.");
                };
            }

            match action {
                ContextAction::Pushdown(new_state) => self.stack.push(new_state),
                ContextAction::Replace(new_state) => {
                    self.stack.pop();
                    self.stack.push(new_state);
                },
                ContextAction::Bounce => {
                    self.stack.pop();
                },
                ContextAction::Clear => self.stack.clear(),
                ContextAction::Retry => (),
            }
        }
        Ok(())
    }

    fn process_global_command(&self, tokens: &Vec<String>) -> Option<ContextAction> {
        match tokens.get(0) {
            Some(a) if a == "quit" => Some(ContextAction::Clear),
            Some(a) if a == "back" => Some(ContextAction::Bounce),
            _ => None,
        }
    }
}
