use crate::input;
use crate::state_machine::{ContextAction, ExitState, State};
use crate::world::World;
use anyhow;

pub struct BuilderRootState {
    pub ship_id: usize,
}

impl State for BuilderRootState {
    fn enter(&self, world: &World) -> anyhow::Result<()> {
        println!(
            "Welcome to {}, the finest purveyor of goods for your spaceship!",
            world.shops[0].name
        );
        println!("How can we help you?");
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> ContextAction {
        match input::get_response_choices(vec!["buy parts", "sell parts", "examine ship", "leave"])
        {
            Ok(0) => ContextAction::Pushdown(Box::new(SelectComponentState {
                ship_id: self.ship_id,
            })),
            Ok(1) => ContextAction::Pushdown(Box::new(SellState {
                ship_id: self.ship_id,
            })),
            Ok(2) => ContextAction::Pushdown(Box::new(ExamineState {
                ship_id: self.ship_id,
            })),
            Ok(3) => ContextAction::Replace(Box::new(ExitState {
                message: "You fly away in your ship and take to the stars!".to_string(),
            })),
            _ => panic!("ohno"),
        }
    }
}

pub struct SelectComponentState {
    pub ship_id: usize,
}

impl State for SelectComponentState {
    fn enter(&self, world: &World) -> anyhow::Result<()> {
        println!("Here's what we have for sale.");
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> ContextAction {
        unimplemented!()
        /*let parts_list = match input::get_response_choices(vec!["Engines", "Weapons"]) {
            Ok(0) => world.shops[0].engine_counts,
            Ok(1) => world.shops[0].weapon_counts,
        }
        ContextAction::Pushdown(Box::new(AddComponentState {
            ship_id: self.ship_id,
        }))*/
    }
}

pub struct SellState {
    pub ship_id: usize,
}

impl State for SellState {
    fn enter(&self, _: &World) -> anyhow::Result<()> {
        unimplemented!()
    }
    fn handle_input(&self, _: &mut World) -> ContextAction {
        unimplemented!()
    }
}

pub struct ExamineState {
    pub ship_id: usize,
}

impl State for ExamineState {
    fn enter(&self, world: &World) -> anyhow::Result<()> {
        println!("Your ship: {}", world.ships[self.ship_id]);
        Ok(())
    }
    fn handle_input(&self, _: &mut World) -> ContextAction {
        ContextAction::Bounce
    }
}
