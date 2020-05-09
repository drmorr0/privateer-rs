pub mod add_component;
pub mod remove_component;
use crate::{
    io,
    state_machine::{
        ship::ExamineShipState,
        ContextAction,
        ResponseType,
        State,
    },
    world::World,
};
use std::cell::RefCell;

#[derive(Clone)]
pub struct BuilderRootState {
    pub ship_id: usize,
    pub shop_id: usize,
    choices: RefCell<Vec<(&'static str, ContextAction)>>,
}

impl BuilderRootState {
    pub fn new(ship_id: usize, shop_id: usize) -> Box<BuilderRootState> {
        Box::new(BuilderRootState {
            ship_id,
            shop_id,
            choices: RefCell::new(Vec::new()),
        })
    }
}

impl State for BuilderRootState {
    fn enter(&self, world: &World) -> ResponseType {
        self.choices.replace(vec![
            (
                "Buy parts",
                ContextAction::Pushdown(add_component::SelectComponentTypeState::new(self.ship_id, self.shop_id)),
            ),
            (
                "Sell parts",
                ContextAction::Pushdown(remove_component::SelectComponentState::new(self.ship_id, self.shop_id)),
            ),
            (
                "Examine ship",
                ContextAction::Pushdown(Box::new(ExamineShipState { ship_id: self.ship_id })),
            ),
        ]);

        println!(
            "Welcome to {}, the finest purveyor of goods for your spaceship!",
            world.shops[self.shop_id].name,
        );
        io::prompt_choices("How can we help you?", &self.choices.borrow());
        ResponseType::Tokenized
    }

    fn transition(&self, tokens: &Vec<String>, _: &mut World) -> Option<ContextAction> {
        io::match_choice(&tokens[0], &self.choices.borrow())
    }
}
