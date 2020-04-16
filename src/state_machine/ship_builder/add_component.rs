use crate::{
    components::{
        Component,
        ComponentType,
    },
    input,
    state_machine::{
        ContextAction,
        State,
    },
    world::World,
};
use anyhow::Result as AnyResult;

// The SelectComponentTypeState is the entry point to the flow which adds new components into a
// ship.  We use it to select the type of component which the player wants to add.
pub struct SelectComponentTypeState {
    pub ship_id: usize,
    pub shop_id: usize,
}

impl State for SelectComponentTypeState {
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
                    ContextAction::Replace(Box::new(SelectComponentState {
                        component_type,
                        ship_id: self.ship_id,
                        shop_id: self.shop_id,
                    })),
                )
            })
            .collect();
        Ok(input::get_response_choices_or_back(
            "What type of components are you interested in?",
            &mut choices,
            ContextAction::Bounce, // action to take on "Back"
        ))
    }
}

struct SelectComponentState {
    component_type: ComponentType,
    ship_id: usize,
    shop_id: usize,
}

impl State for SelectComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        Ok(())
    }
    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let available_components = world.shops[self.shop_id].available_sorted_components(self.component_type);
        let mut choices = available_components
            .iter()
            .map(|(component_id, component, available)| {
                (
                    format!(
                        "{} ({} available, {} slots)",
                        component.name(),
                        available,
                        component.slots()
                    ),
                    ContextAction::Replace(Box::new(SelectLocationState {
                        component_type: self.component_type,
                        component_id: *component_id,
                        component: *component,
                        ship_id: self.ship_id,
                        shop_id: self.shop_id,
                    })),
                )
            })
            .collect();

        Ok(input::get_response_choices_or_back(
            &format!("Ok, we have the following {}", self.component_type.to_plural()),
            &mut choices,
            ContextAction::Bounce,
        ))
    }
}

struct SelectLocationState {
    component_type: ComponentType,
    component_id: usize,
    component: &'static dyn Component,
    ship_id: usize,
    shop_id: usize,
}

impl State for SelectLocationState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let ship = &mut world.ships[self.ship_id];
        let mut segments = ship
            .segments()
            .map(|(location_id, segment)| {
                (
                    format!(
                        "{} ({}/{} slots used)",
                        &segment.name, &segment.used_slots, &segment.slots
                    ),
                    ContextAction::Replace(Box::new(AddComponentState {
                        component_type: self.component_type,
                        component_id: self.component_id,
                        component: self.component,
                        location_id,
                        ship_id: self.ship_id,
                        shop_id: self.shop_id,
                    })),
                )
            })
            .collect();
        Ok(input::get_response_choices_or_back(
            &format!("Where do you want to install the {}", self.component.name()),
            &mut segments,
            ContextAction::Bounce,
        ))
    }
}

struct AddComponentState {
    component_type: ComponentType,
    component_id: usize,
    component: &'static dyn Component,
    location_id: usize,
    ship_id: usize,
    shop_id: usize,
}

impl State for AddComponentState {
    fn enter(&self, _: &World) -> AnyResult<()> {
        Ok(())
    }

    fn handle_input(&self, world: &mut World) -> AnyResult<ContextAction> {
        let ship = &mut world.ships[self.ship_id];
        match ship.add_component(self.component.clone(), self.location_id) {
            Ok(slots_remaining) => {
                world.shops[self.shop_id].take_component(self.component_id, self.component_type);
                println!(
                    "We installed the {} in the {}; you have {} slots remaining in that segment.",
                    self.component.name(),
                    ship.segment(self.location_id).name,
                    slots_remaining,
                );
            },
            Err(slots_remaining) => println!(
                "Sorry, you don't have enough room to install a {} in the {}; it only has {} slots remaining.",
                self.component.name(),
                ship.segment(self.location_id).name,
                slots_remaining,
            ),
        }
        Ok(ContextAction::Bounce)
    }
}
