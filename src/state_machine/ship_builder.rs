use crate::components::Component;
use crate::input;
use crate::state_machine::{ContextAction, ExitState, State};
use crate::world::World;
use anyhow::anyhow;
use anyhow::Result as AnyResult;

pub struct BuilderRootState {
    pub ship_id: usize,
}

impl State for BuilderRootState {
    fn enter(&self, world: &World) -> AnyResult<()> {
        println!(
            "Welcome to {}, the finest purveyor of goods for your spaceship!",
            world.shops[0].name
        );
        println!("How can we help you?");
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        match input::get_response_choices(&vec!["buy parts", "sell parts", "examine ship", "leave"])
        {
            Ok(0) => Ok(ContextAction::Pushdown(Box::new(SelectComponentState {
                ship_id: self.ship_id,
            }))),
            Ok(1) => Ok(ContextAction::Pushdown(Box::new(SellState {
                ship_id: self.ship_id,
            }))),
            Ok(2) => Ok(ContextAction::Pushdown(Box::new(ExamineState {
                ship_id: self.ship_id,
            }))),
            Ok(3) => Ok(ContextAction::Replace(Box::new(ExitState {
                message: "You fly away in your ship and take to the stars!".to_string(),
            }))),
            _ => Err(anyhow!("ohno")),
        }
    }
}

pub struct SelectComponentState {
    pub ship_id: usize,
}

impl State for SelectComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        println!("Here's what we have for sale.");
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let choices = vec!["Engines", "Weapons"];
        let idx = input::get_response_choices(&choices)?;
        Ok(ContextAction::Pushdown(Box::new(AddComponentState {
            ship_id: self.ship_id,
            component_type: choices[idx].to_string(),
            available_components: match choices[idx] {
                "Engines" => world.shops[0].available_engines(),
                _ => unimplemented!(),
            },
        })))
    }
}

pub struct AddComponentState {
    pub ship_id: usize,
    component_type: String,
    available_components: Vec<(&'static dyn Component, u32)>,
}

impl State for AddComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        println!("Ok, we have the following {}", self.component_type);
        for (i, (component, count)) in self.available_components.iter().enumerate() {
            println!("  [{}] {}: {} available", i + 1, component.name(), count);
        }
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let u: Vec<usize> = (1..self.available_components.len() + 1).collect();
        let choice = input::get_response("", &u)?;
        world.ships[self.ship_id]
            .add_component(self.available_components[choice - 1].0.clone(), "Fuselage")
            .unwrap();
        Ok(ContextAction::Bounce)
    }
}

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

pub struct ExamineState {
    pub ship_id: usize,
}

impl State for ExamineState {
    fn enter(&self, world: &World) -> AnyResult<()> {
        println!("Your ship: {}", world.ships[self.ship_id]);
        Ok(())
    }
    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        Ok(ContextAction::Bounce)
    }
}
