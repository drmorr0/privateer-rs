use crate::{
    input,
    state_machine::{
        ContextAction,
        State,
    },
    world::World,
};
use anyhow::Result as AnyResult;

pub struct SelectComponentState {
    pub ship_id: usize,
    pub shop_id: usize,
}

impl State for SelectComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        println!("We can offer you the best deals in the galaxy on your used parts!");
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let ship = &world.ships[self.ship_id];
        let mut choices = ship
            .segments()
            .map(|(_, segment)| {
                segment.component_ids.iter().map(|&component_id| {
                    (
                        format!("{}", ship.components[&component_id].name()),
                        ContextAction::Replace(Box::new(SellComponentState {
                            component_id,
                            ship_id: self.ship_id,
                            shop_id: self.shop_id,
                        })),
                    )
                })
            })
            .flatten()
            .collect();
        Ok(input::get_response_choices_or_back(
            "What would you like to sell?",
            &mut choices,
            ContextAction::Bounce,
        ))
    }
}

struct SellComponentState {
    component_id: usize,
    ship_id: usize,
    shop_id: usize,
}

impl State for SellComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let ship = &mut world.ships[self.ship_id];
        if input::get_response_yn(&format!(
            "Are you sure you want to sell your {}?",
            ship.components[&self.component_id].name()
        )) {
            let c = ship.remove_component(self.component_id);
            world.shops[self.shop_id].gain_component(c);
        }
        Ok(ContextAction::Bounce)
    }
}
