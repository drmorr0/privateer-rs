use crate::components;
use ron::de;
use std::fs::File;
use std::io::BufReader;

fn read_template_file<T: for<'de> serde::Deserialize<'de>>(filename: String) -> Vec<T> {
    let f = match File::open(&filename) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not open {}:\n  {}", &filename, e);
            panic!();
        }
    };
    let reader = BufReader::new(f);
    let res: Vec<T> = match de::from_reader(reader) {
        Ok(f) => f,
        Err(e) => {
            println!("Could not parse RON-file {}:\n  {}", &filename, e);
            panic!();
        }
    };
    res
}

pub struct TemplateStore {
    pub hull_templates: Vec<components::HullData>,
    pub engine_templates: Vec<components::EngineData>,
}

impl TemplateStore {
    pub fn new() -> TemplateStore {
        TemplateStore {
            hull_templates: read_template_file("./data/hulls.ron".to_string()),
            engine_templates: read_template_file("./data/engines.ron".to_string()),
        }
    }
}
