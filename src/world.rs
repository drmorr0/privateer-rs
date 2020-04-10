use crate::components::{Component, Hull};
use crate::ship::Ship;
use crate::template::TemplateStore;

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

fn sort_components(
    complist: &Vec<u32>,
    comp_getter: fn(usize) -> &'static dyn Component,
) -> Vec<(&'static dyn Component, u32)> {
    let mut res: Vec<(&'static dyn Component, u32)> = complist
        .iter()
        .enumerate()
        .filter_map(|(id, &count)| match count {
            x if x > 0 => Some((comp_getter(id), x)),
            _ => None,
        })
        .collect();
    res.sort_by(|a, b| a.0.name().cmp(b.0.name()));
    res
}

impl Shop {
    pub fn available_engines(&self) -> Vec<(&'static dyn Component, u32)> {
        sort_components(&self.engine_counts, TemplateStore::engine)
    }

    pub fn available_weapons(&self) -> Vec<(&'static dyn Component, u32)> {
        sort_components(&self.weapon_counts, TemplateStore::weapon)
    }
}

/*impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Engines:")?;
        writeln!(f, "--------")?;

        for (i, (id, count)) in self.engine_counts.iter().enumerate() {
            let component = TemplateStore::engine(*id).unwrap();
            writeln!(f, "  [{}] {}: {}", i, component.name(), count)?;
        }
        Ok(())
    }
}*/
