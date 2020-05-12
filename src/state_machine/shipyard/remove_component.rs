use crate::{
    io,
    state_machine::{
        ContextAction,
        ResponseType,
        State,
    },
    world::World,
};
use std::cell::RefCell;

#[derive(Clone)]
pub struct SelectComponentState {
    ship_id: usize,
    shop_id: usize,
    choices: RefCell<Vec<(String, ContextAction)>>,
}

impl SelectComponentState {
    pub fn new(ship_id: usize, shop_id: usize) -> Box<SelectComponentState> {
        Box::new(SelectComponentState {
            ship_id,
            shop_id,
            choices: RefCell::new(Vec::new()),
        })
    }
}

impl State for SelectComponentState {
    fn enter(&self, world: &World) -> ResponseType {
        let ship = &world.ships[self.ship_id];
        self.choices.replace(
            ship.segments()
                .map(|(_, segment)| {
                    segment.component_ids.iter().map(|&component_id| {
                        (
                            format!("{}", ship.components[&component_id].name()),
                            ContextAction::Replace(SellComponentState::new(self.ship_id, self.shop_id, component_id)),
                        )
                    })
                })
                .flatten()
                .collect(),
        );
        println!("We can offer you the best deals in the galaxy on your used parts!");
        io::prompt_choices("What would you like to sell?", &self.choices.borrow());
        ResponseType::Tokenized
    }

    fn transition(&self, tokens: &[String], _: &mut World) -> Option<ContextAction> {
        io::match_choice(&tokens[0], &self.choices.borrow())
    }
}

#[derive(Clone)]
struct SellComponentState {
    ship_id: usize,
    shop_id: usize,
    component_id: usize,
}

impl SellComponentState {
    pub fn new(ship_id: usize, shop_id: usize, component_id: usize) -> Box<SellComponentState> {
        Box::new(SellComponentState {
            ship_id,
            shop_id,
            component_id,
        })
    }
}

impl State for SellComponentState {
    fn enter(&self, world: &World) -> ResponseType {
        let ship = &world.ships[self.ship_id];
        println!(
            "Are you sure you want to sell your {}?",
            ship.components[&self.component_id].name(),
        );
        ResponseType::Raw
    }

    fn transition(&self, tokens: &[String], world: &mut World) -> Option<ContextAction> {
        let ship = &mut world.ships[self.ship_id];
        if let Some(sell) = io::match_response_yn(&tokens[0]) {
            if sell {
                let c = ship.remove_component(self.component_id);
                world.shops[self.shop_id].gain_component(c);
            }
            Some(ContextAction::Bounce)
        } else {
            None
        }
    }
}
