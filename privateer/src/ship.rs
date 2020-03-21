use crate::components::{Hull};
use std::collections::HashMap;
use std::fmt;

pub struct Ship {
    pub name: String,
    hull: Hull,
/*    weapons: Vec<Weapon>,
    engines: Vec<Engine>,
    entities: Vec<Entity>,*/
}

impl Ship {
    pub fn new(name: String, hull_template: Hull) {
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {:?} {})",
            self.name, self.hull.name, self.hull.class, self.hull.role
        )
    }
}
