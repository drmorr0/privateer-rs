use crate::components::{Component, Engine, Hull, Weapon};
use anyhow;
use erased_serde;
use lazy_static::lazy_static;
use ron::de::Deserializer;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

lazy_static! {
    static ref TEMPLATE_STORE: TemplateStore = TemplateStore::new();
}

fn read_template_file<T: Component + for<'de> Deserialize<'de>>(
    filename: String,
) -> anyhow::Result<HashMap<String, T>> {
    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open {}:\n  {}", &filename, e);
            panic!();
        }
    };
    let mut reader = BufReader::new(f);
    let mut tmpl_str = String::new();
    reader.read_to_string(&mut tmpl_str)?;
    let ron_deserializer = &mut Deserializer::from_str(&tmpl_str)?;
    let mut ron_deserializer = erased_serde::Deserializer::erase(ron_deserializer);
    let mut res: Vec<T> = match erased_serde::deserialize(&mut ron_deserializer) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not parse RON-file {}:\n  {}", &filename, e);
            panic!();
        }
    };
    let mut hash_res: HashMap<String, T> = HashMap::new();
    loop {
        match res.pop() {
            Some(el) => {
                hash_res.insert(el.name().to_string(), el);
            }
            None => break,
        }
    }
    Ok(hash_res)
}

pub struct TemplateStore {
    hull_templates: HashMap<String, Hull>,
    engine_templates: HashMap<String, Engine>,
    weapon_templates: HashMap<String, Weapon>,
}

impl TemplateStore {
    fn new() -> TemplateStore {
        TemplateStore {
            hull_templates: read_template_file("./data/hulls.ron".to_string()).unwrap(),
            engine_templates: read_template_file("./data/engines.ron".to_string()).unwrap(),
            weapon_templates: HashMap::new(), //read_template_file("./data/weapons.ron".to_string()).unwrap(),
        }
    }

    pub fn hulls() -> &'static HashMap<String, Hull> {
        &TEMPLATE_STORE.hull_templates
    }

    pub fn engines() -> &'static HashMap<String, Engine> {
        &TEMPLATE_STORE.engine_templates
    }

    pub fn weapons() -> &'static HashMap<String, Weapon> {
        &TEMPLATE_STORE.weapon_templates
    }
}
