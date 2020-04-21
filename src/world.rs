use crate::{
    components::{
        Component,
        ComponentType,
        Hull,
    },
    ship::Ship,
    template::TemplateStore,
    util::enumiter,
};

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

    pub fn mk_ship(&mut self, name: &str, hull: Hull) -> usize {
        let id = self.ships.len();
        self.ships.push(Ship::new(name.to_string(), id, &hull));
        id
    }

    fn mk_shop(&mut self, name: String) {
        self.shops.push(Shop {
            name,
            engine_counts: vec![5; TemplateStore::engine_count()],
            weapon_counts: vec![5; TemplateStore::weapon_count()],
        });
    }
}

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
