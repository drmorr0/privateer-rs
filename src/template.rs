use crate::components::{
    Component,
    Engine,
    Hull,
    Weapon,
};
use anyhow::Result as AnyResult;
use erased_serde;
use lazy_static::lazy_static;
use ron::de::Deserializer;
use serde::Deserialize;
use std::{
    fs::File,
    io::{
        BufReader,
        Read,
    },
};

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
            hull_templates: read_template_file("./data/hulls.ron".to_string()).unwrap(),
            engine_templates: read_template_file("./data/engines.ron".to_string()).unwrap(),
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

fn read_template_file<T: Component + for<'de> Deserialize<'de>>(filename: String) -> AnyResult<Vec<T>> {
    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open {}:\n  {}", &filename, e);
            panic!();
        },
    };
    let mut reader = BufReader::new(f);
    let mut tmpl_str = String::new();
    reader.read_to_string(&mut tmpl_str)?;
    let ron_deserializer = &mut Deserializer::from_str(&tmpl_str)?;
    let mut ron_deserializer = erased_serde::Deserializer::erase(ron_deserializer);
    let templates: Vec<T> = match erased_serde::deserialize(&mut ron_deserializer) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not parse RON-file {}:\n  {}", &filename, e);
            panic!();
        },
    };
    Ok(templates)
}
