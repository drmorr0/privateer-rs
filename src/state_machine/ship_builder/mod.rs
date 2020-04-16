pub mod add_component;
pub mod remove_component;
use crate::{
    input,
    state_machine::{
        ship::ExamineShipState,
        ship_builder::{
            add_component::SelectComponentTypeState,
            remove_component::SellState,
        },
        ContextAction,
        State,
    },
    world::World,
};
use anyhow::Result as AnyResult;

pub struct BuilderRootState {
    pub ship_id: usize,
    pub shop_id: usize,
}

impl State for BuilderRootState {
    fn enter(&self, world: &World) -> AnyResult<()> {
        println!(
            "Welcome to {}, the finest purveyor of goods for your spaceship!",
            world.shops[self.shop_id].name
        );
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        Ok(input::get_response_choices_or_back(
            "How can we help you?",
            &mut vec![
                (
                    "Buy parts",
                    ContextAction::Pushdown(Box::new(SelectComponentTypeState {
                        ship_id: self.ship_id,
                        shop_id: self.shop_id,
                    })),
                ),
                (
                    "Sell parts",
                    ContextAction::Pushdown(Box::new(SellState { ship_id: self.ship_id })),
                ),
                (
                    "Examine ship",
                    ContextAction::Pushdown(Box::new(ExamineShipState { ship_id: self.ship_id })),
                ),
            ],
            ContextAction::Bounce, // action to take on "Back"
        ))
    }
}
