use crate::components::HullData;
use crate::ship::Ship;
use crate::template::TemplateStore;
use std::collections::HashMap;
use std::fmt;

pub struct World {
    pub ships: Vec<Ship>,
    pub shops: Vec<Shop>,
    pub template_store: TemplateStore,
}

impl World {
    pub fn new() -> World {
        let mut w = World {
            ships: vec![],
            shops: vec![],
            template_store: TemplateStore::new(),
        };
        w.mk_shop("A Better, Cheaper, Shipsmith".to_string());
        w
    }

    pub fn mk_ship(&mut self, name: String, hull: HullData) -> usize {
        let id = self.ships.len();
        self.ships.push(Ship::new(name, id, hull));
        id
    }

    pub fn mk_shop(&mut self, name: String) {
        let engine_counts = self
            .template_store
            .engine_templates
            .iter()
            .map(|t| (t.name.clone(), 5))
            .collect();
        self.shops.push(Shop {
            name,
            engine_counts,
        });
    }
}

pub struct Shop {
    pub name: String,
    pub engine_counts: HashMap<String, u32>,
}

impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Engines:")?;
        writeln!(f, "--------")?;

        for (i, (name, count)) in self.engine_counts.iter().enumerate() {
            writeln!(f, "  [{}] {}: {}", i, name, count)?;
        }
        Ok(())
    }
}
