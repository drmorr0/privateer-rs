use crate::{
    components::{
        Component,
        ComponentType,
        Hull,
    },
    io,
    ship::Ship,
    template::TemplateStore,
    util::enumiter,
};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Deserialize, Serialize)]
pub struct World {
    pub ships: Vec<Ship>,
    pub shops: Vec<Shop>,
}

impl<'wld> World {
    pub fn new() -> World {
        World {
            ships: vec![],
            shops: vec![],
        }
    }

    pub fn load(filename: &str) -> World {
        io::read_data_file(filename)
    }

    pub fn mk_ship(&mut self, name: &str, hull: Hull) -> usize {
        let id = self.ships.len();
        self.ships.push(Ship::new(name.to_string(), id, &hull));
        id
    }

    pub fn mk_shop(&mut self, name: &str) {
        self.shops.push(Shop {
            name: name.to_string(),
            engine_counts: vec![100; TemplateStore::engine_count()],
            weapon_counts: vec![100; TemplateStore::weapon_count()],
        });
    }
}

#[derive(Deserialize, Serialize)]
pub struct Shop {
    pub name: String,
    pub engine_counts: Vec<u32>,
    pub weapon_counts: Vec<u32>,
}

pub fn sort_components(
    complist: &Vec<u32>,
    comp_getter: fn(usize) -> &'static dyn Component,
) -> Vec<(usize, &'static dyn Component, u32)> {
    let mut res: Vec<(usize, &'static dyn Component, u32)> = enumiter(complist)
        .filter_map(|(id, &count)| match count {
            x if x > 0 => Some((id, comp_getter(id), x)),
            _ => None,
        })
        .collect();
    res.sort_by(|a, b| a.1.name().cmp(b.1.name()));
    res
}

impl Shop {
    pub fn available_sorted_components(&self, ctype: ComponentType) -> Vec<(usize, &'static dyn Component, u32)> {
        match ctype {
            ComponentType::Hull(_) => vec![],
            ComponentType::Engine(_) => sort_components(&self.engine_counts, TemplateStore::engine),
            ComponentType::Weapon(_) => sort_components(&self.weapon_counts, TemplateStore::weapon),
        }
    }

    pub fn take_component(&mut self, ctype: ComponentType) {
        match ctype {
            ComponentType::Hull(_) => (),
            ComponentType::Engine(id) => self.engine_counts[id] -= 1,
            ComponentType::Weapon(id) => self.weapon_counts[id] -= 1,
        }
    }

    pub fn gain_component(&mut self, component: Box<dyn Component>) {
        match component.ctype() {
            ComponentType::Hull(_) => (),
            ComponentType::Engine(id) => self.engine_counts[id] += 1,
            ComponentType::Weapon(id) => self.weapon_counts[id] += 1,
        }
    }
}
