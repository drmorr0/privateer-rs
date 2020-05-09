use crate::{
    components::{
        Component,
        Engine,
        Hull,
        Weapon,
    },
    io,
};
use lazy_static::lazy_static;

lazy_static! {
    static ref TEMPLATE_STORE: TemplateStore = TemplateStore::new();
}

pub struct TemplateStore {
    hull_templates: Vec<Hull>,
    engine_templates: Vec<Engine>,
    weapon_templates: Vec<Weapon>,
}

impl TemplateStore {
    fn new() -> TemplateStore {
        TemplateStore {
            hull_templates: io::read_data_file("./data/hulls.ron"),
            engine_templates: io::read_data_file("./data/engines.ron"),
            weapon_templates: Vec::new(), //read_template_file("./data/weapons.ron".to_string()).unwrap(),
        }
    }

    pub fn hull(id: usize) -> Option<&'static Hull> {
        TEMPLATE_STORE.hull_templates.get(id)
    }

    pub fn engine(id: usize) -> &'static dyn Component {
        let e = TEMPLATE_STORE.engine_templates.get(id).unwrap();
        e as &'static dyn Component
    }

    pub fn engine_count() -> usize {
        TEMPLATE_STORE.engine_templates.len()
    }

    pub fn weapon(id: usize) -> &'static dyn Component {
        let w = TEMPLATE_STORE.weapon_templates.get(id).unwrap();
        w as &'static dyn Component
    }

    pub fn weapon_count() -> usize {
        TEMPLATE_STORE.weapon_templates.len()
    }
}
