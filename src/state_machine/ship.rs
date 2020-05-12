use crate::{
    state_machine::{
        ContextAction,
        ResponseType,
        State,
    },
    world::World,
};

#[derive(Clone)]
pub struct ExamineShipState {
    pub ship_id: usize,
}

impl State for ExamineShipState {
    fn enter(&self, world: &World) -> ResponseType {
        println!("Your ship: {}", world.ships[self.ship_id]);
        ResponseType::None
    }

    fn transition(&self, _: &[String], _: &mut World) -> Option<ContextAction> {
        Some(ContextAction::Bounce)
    }
}
