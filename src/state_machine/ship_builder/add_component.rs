use crate::{
    components::{
        make_ctype_with_id,
        Component,
        ComponentType,
    },
    input,
    state_machine::{
        ContextAction,
        ResponseType,
        State,
    },
    world::World,
};
use std::cell::RefCell;

// The SelectComponentTypeState is the entry point to the flow which adds new components into a
// ship.  We use it to select the type of component which the player wants to add.
#[derive(Clone)]
pub struct SelectComponentTypeState {
    ship_id: usize,
    shop_id: usize,
    choices: RefCell<Vec<(String, ContextAction)>>,
}

impl SelectComponentTypeState {
    pub fn new(ship_id: usize, shop_id: usize) -> Box<SelectComponentTypeState> {
        Box::new(SelectComponentTypeState {
            ship_id,
            shop_id,
            choices: RefCell::new(Vec::new()),
        })
    }
}

impl State for SelectComponentTypeState {
    fn enter(&self, _: &World) -> ResponseType {
        println!("Here's what we have for sale.");
        let ctypes = vec![
            // The actual ID here doesn't matter, if it ever gets used we'll panic
            ComponentType::Engine(usize::max_value()),
            ComponentType::Weapon(usize::max_value()),
        ];
        self.choices.replace(
            ctypes
                .iter()
                .map(|&ctype| {
                    (
                        ctype.to_plural(),
                        ContextAction::Replace(SelectComponentState::new(self.ship_id, self.shop_id, ctype)),
                    )
                })
                .collect(),
        );
        input::prompt_choices("What type of components are you interested in?", &self.choices.borrow());
        ResponseType::Raw
    }

    fn transition(&self, tokens: &Vec<String>, _: &mut World) -> Option<ContextAction> {
        input::match_choice(&tokens[0], &self.choices.borrow())
    }
}

#[derive(Clone)]
struct SelectComponentState {
    ship_id: usize,
    shop_id: usize,
    ctype: ComponentType,
    available_components: RefCell<Vec<(usize, &'static dyn Component, u32)>>,
    choices: RefCell<Vec<(String, ContextAction)>>,
}

impl SelectComponentState {
    pub fn new(ship_id: usize, shop_id: usize, ctype: ComponentType) -> Box<SelectComponentState> {
        Box::new(SelectComponentState {
            ship_id,
            shop_id,
            ctype,
            available_components: RefCell::new(Vec::new()),
            choices: RefCell::new(Vec::new()),
        })
    }
}

impl State for SelectComponentState {
    fn enter(&self, world: &World) -> ResponseType {
        self.available_components
            .replace(world.shops[self.shop_id].available_sorted_components(self.ctype));
        self.choices.replace(
            self.available_components
                .borrow()
                .iter()
                .map(|(component_id, component, available)| {
                    (
                        format!(
                            "{} ({} available, {} slots)",
                            component.name(),
                            available,
                            component.slots()
                        ),
                        ContextAction::Replace(SelectLocationState::new(
                            self.ship_id,
                            self.shop_id,
                            make_ctype_with_id(self.ctype, *component_id),
                            *component,
                        )),
                    )
                })
                .collect(),
        );

        input::prompt_choices(
            &format!("Ok, we have the following {}", self.ctype.to_plural()),
            &self.choices.borrow(),
        );
        ResponseType::Tokenized
    }

    fn transition(&self, tokens: &Vec<String>, _: &mut World) -> Option<ContextAction> {
        if tokens.len() > 1 {
            match input::match_command_choice("inspect", tokens, &self.available_components.borrow()) {
                Some(c) => {
                    println!("\n{}", c.1);
                    Some(ContextAction::Retry)
                },
                _ => None,
            }
        } else {
            return input::match_choice(&tokens[0], &self.choices.borrow());
        }
    }
}

#[derive(Clone)]
struct SelectLocationState {
    ship_id: usize,
    shop_id: usize,
    ctype: ComponentType,
    component: &'static dyn Component,
    choices: RefCell<Vec<(String, usize)>>,
}

impl SelectLocationState {
    pub fn new(
        ship_id: usize,
        shop_id: usize,
        ctype: ComponentType,
        component: &'static dyn Component,
    ) -> Box<SelectLocationState> {
        Box::new(SelectLocationState {
            ship_id,
            shop_id,
            ctype,
            component,
            choices: RefCell::new(Vec::new()),
        })
    }
}

impl State for SelectLocationState {
    fn enter(&self, world: &World) -> ResponseType {
        let ship = &world.ships[self.ship_id];
        self.choices.replace(
            ship.segments()
                .map(|(location_id, segment)| {
                    (
                        format!(
                            "{} ({}/{} slots used)",
                            &segment.name, &segment.used_slots, &segment.slots
                        ),
                        location_id,
                    )
                })
                .collect(),
        );

        input::prompt_choices(
            &format!("Where do you want to install the {}", self.component.name()),
            &self.choices.borrow(),
        );
        ResponseType::Tokenized
    }

    fn transition(&self, tokens: &Vec<String>, world: &mut World) -> Option<ContextAction> {
        if let Some(location_id) = input::match_choice(&tokens[0], &self.choices.borrow()) {
            let ship = &mut world.ships[self.ship_id];
            match ship.add_component(self.component.clone(), location_id) {
                Ok(slots_remaining) => {
                    world.shops[self.shop_id].take_component(self.ctype);
                    println!(
                        "We installed the {} in the {}; you have {} slots remaining in that segment.",
                        self.component.name(),
                        ship.segment(location_id).name,
                        slots_remaining,
                    );
                },
                Err(slots_remaining) => println!(
                    "Sorry, you don't have enough room to install a {} in the {}; it only has {} slots remaining.",
                    self.component.name(),
                    ship.segment(location_id).name,
                    slots_remaining,
                ),
            }
            Some(ContextAction::Bounce)
        } else {
            None
        }
    }
}
