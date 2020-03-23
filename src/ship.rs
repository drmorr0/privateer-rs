use crate::components::*;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub name: String,
    components: Vec<ComponentType>,
}

impl Ship {
    pub fn new(name: String, hull_template: HullData) -> Ship {
        Ship {
            name,
            components: vec![ComponentType::Hull(hull_template)],
        }
    }

    pub fn hull(&self) -> &HullData {
        match &self.components[0] {
            ComponentType::Hull(d) => d,
            _ => panic!("This ship has no hull!"),
        }
    }

    pub fn mass(&self) -> u32 {
        self.components.iter().fold(0, |m, x| m + mass(&x))
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {:?} {})",
            self.name, self.hull().name, self.hull().class, self.hull().role
        )
    }
}
