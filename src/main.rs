mod components;
mod io;
mod ship;
mod state_machine;
mod template;
mod util;
mod world;

use crate::world::World;
use std::{
    env,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let entry_state = state_machine::EntryState::new();
    let world = if args.len() > 1 {
        Some(World::load(&args[1]))
    } else {
        None
    };
    let mut context = state_machine::Context::new(world, entry_state);
    context.run()?;
    Ok(())
}
