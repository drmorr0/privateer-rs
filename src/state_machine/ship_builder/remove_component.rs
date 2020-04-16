use crate::{
    state_machine::{
        ContextAction,
        State,
    },
    world::World,
};
use anyhow::Result as AnyResult;

pub struct SellState {
    pub ship_id: usize,
}

impl State for SellState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        unimplemented!()
    }
    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        unimplemented!()
    }
}
