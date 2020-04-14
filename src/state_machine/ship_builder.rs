use crate::{
    components::ComponentType,
    input,
    state_machine::{
        ContextAction,
        State,
    },
    world::World,
};
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
        Ok(())
    }

    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        Ok(input::get_response_choices(
            "How can we help you?",
            &mut vec![
                (
                    "buy parts",
                    ContextAction::Pushdown(Box::new(SelectComponentState { ship_id: self.ship_id })),
                ),
                (
                    "sell parts",
                    ContextAction::Pushdown(Box::new(SellState { ship_id: self.ship_id })),
                ),
                (
                    "examine ship",
                    ContextAction::Pushdown(Box::new(ExamineState { ship_id: self.ship_id })),
                ),
                ("leave", ContextAction::Bounce),
            ],
        ))
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
    fn handle_input(&self, _: &mut World) -> AnyResult<ContextAction> {
        let choices = vec![ComponentType::Engine, ComponentType::Weapon];
        let mut choices = choices
            .iter()
            .map(|&component_type| {
                (
                    component_type.to_plural(),
                    ContextAction::Pushdown(Box::new(AddComponentState {
                        component_type,
                        ship_id: self.ship_id,
                        shop_id: 0,
                    })),
                )
            })
            .collect();
        Ok(input::get_response_choices(
            "What type of components are you interested in?",
            &mut choices,
        ))
    }
}

pub struct AddComponentState {
    component_type: ComponentType,
    ship_id: usize,
    shop_id: usize,
}

impl State for AddComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let available_components = world.shops[self.shop_id].available_sorted_components(self.component_type);
        let (&choice_id, choice) = input::get_response_choices(
            &format!("Ok, we have the following {}", self.component_type.to_plural()),
            &mut available_components
                .iter()
                .map(|(id, comp, available)| {
                    (
                        format!("{} ({} available, {} slots)", comp.name(), available, comp.slots()),
                        (id, comp),
                    )
                })
                .collect(),
        );
        let ship = &mut world.ships[self.ship_id];
        let mut segments: Vec<(String, usize)> = ship
            .segments()
            .map(|(id, segment)| {
                (
                    format!(
                        "{} ({}/{} slots used)",
                        &segment.name, &segment.used_slots, &segment.slots
                    ),
                    id,
                )
            })
            .collect();
        let location = input::get_response_choices(
            &format!("Where do you want to install the {}", choice.name()),
            &mut segments,
        );
        match ship.add_component(choice.clone(), location) {
            Ok(slots_remaining) => {
                world.shops[self.shop_id].take_component(choice_id, self.component_type);
                println!(
                    "We installed the {} in the {}; you have {} slots remaining in that segment.",
                    choice.name(),
                    ship.segment(location).name,
                    slots_remaining,
                );
            },
            Err(slots_remaining) => println!(
                "Sorry, you don't have enough room to install a {} in the {}; it only has {} slots remaining.",
                choice.name(),
                ship.segment(location).name,
                slots_remaining,
            ),
        }
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
