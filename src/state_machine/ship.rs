use crate::{
    state_machine::{
        ContextAction,
        State,
    },
    world::World,
};
use anyhow::Result as AnyResult;

pub struct ExamineShipState {
    pub ship_id: usize,
}

impl State for ExamineShipState {
    fn enter(&self, world: &World) -> AnyResult<()> {
        println!("Your ship: {}", world.ships[self.ship_id]);
        Ok(())
    }
    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        Ok(ContextAction::Bounce)
    }
}
