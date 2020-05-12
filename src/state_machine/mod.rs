pub mod ship;
pub mod shipyard;
mod util;
use crate::world::World;
use anyhow::Result as AnyResult;
use std::{
    collections::HashMap,
    io::{
        self,
        Write,
    },
};

pub type CommandFunction = fn(&[String], &World) -> Option<ContextAction>;

#[derive(Clone)] // These get cloned during the input::match_choice routine
pub enum ContextAction {
    Pushdown(Box<dyn State>),
    Replace(Box<dyn State>),
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
    fn transition(&self, tokens: &[String], world: &mut World) -> Option<ContextAction>;
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

    fn transition(&self, _: &[String], _: &mut World) -> Option<ContextAction> {
        Some(ContextAction::Clear)
    }
}

pub struct Context {
    stack: Vec<Box<dyn State>>,
    world: Box<World>,
    commands: HashMap<String, CommandFunction>,
}

impl Context {
    pub fn new(world: World, starting_state: Box<dyn State>, commands: HashMap<String, CommandFunction>) -> Context {
        Context {
            stack: vec![starting_state],
            world: Box::new(world),
            commands,
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

                if let Some(a) = self.process_global_command(&tokens, &self.world) {
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

    fn process_global_command(&self, tokens: &Vec<String>, world: &World) -> Option<ContextAction> {
        match tokens.get(0) {
            Some(a) if self.commands.contains_key(a) => self.commands[a](&tokens[1..], world),
            Some(a) if a == "quit" => Some(ContextAction::Clear),
            Some(a) if a == "back" => Some(ContextAction::Bounce),
            _ => None,
        }
    }
}
