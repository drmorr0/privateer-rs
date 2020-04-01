use crate::state_machine::{State, StatePointer};
use crate::world::World;
use anyhow;

pub struct BuilderRootState { }

impl State for BuilderRootState {
    fn enter(&self, world: &World) -> anyhow::Result<()> {
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> StatePointer {
        Err(anyhow::anyhow!("foo"))
    }
}
