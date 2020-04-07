use crate::components::{Component, Hull};
use crate::ship::Ship;
use crate::template::TemplateStore;
use std::collections::HashMap;
use std::fmt;

pub struct World {
    pub ships: Vec<Ship>,
    pub shops: Vec<Shop>,
}

impl<'wld> World {
    pub fn new() -> World {
        let mut w = World {
            ships: vec![],
            shops: vec![],
        };
        w.mk_shop("A Better, Cheaper, Shipsmith".to_string());
        w
    }

    pub fn mk_ship(&mut self, name: String, hull: Hull) -> usize {
        let id = self.ships.len();
        self.ships.push(Ship::new(name, id, &hull));
        id
    }

    pub fn mk_shop(&mut self, name: String) {
        let mut engine_counts: HashMap<&str, u32> = HashMap::new();
        let mut weapon_counts: HashMap<&str, u32> = HashMap::new();
        for tmpl in TemplateStore::engines().values() {
            engine_counts.insert(tmpl.name(), 7);
        }
        for tmpl in TemplateStore::weapons().values() {
            weapon_counts.insert(tmpl.name(), 5);
        }
        self.shops.push(Shop {
            name,
            engine_counts,
            weapon_counts,
        });
    }
}

pub struct Shop {
    pub name: String,
    pub engine_counts: HashMap<&'static str, u32>,
    pub weapon_counts: HashMap<&'static str, u32>,
}

impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Engines:")?;
        writeln!(f, "--------")?;

        for (i, (component_name, count)) in self.engine_counts.iter().enumerate() {
            writeln!(f, "  [{}] {}: {}", i, component_name, count)?;
        }
        Ok(())
    }
}
