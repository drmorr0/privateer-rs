use crate::components::HullData;
use crate::ship::Ship;
use crate::template::TemplateStore;

pub struct World {
    pub ships: Vec<Ship>,
    pub templateStore: TemplateStore,
}

impl World {
    pub fn new() -> World {
        World {
            ships: vec![],
            templateStore: TemplateStore::new(),
        }
    }

    pub fn mk_ship(&mut self, name: String, hull: HullData) -> usize {
        let id = self.ships.len();
        self.ships.push(Ship::new(name, id, hull));
        id
    }
}
