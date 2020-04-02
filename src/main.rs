mod components;
mod input;
mod ship;
mod state_machine;
mod template;
mod world;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = state_machine::Context::new(Box::new(state_machine::EntryState {}));
    context.run()?;
    Ok(())
}
