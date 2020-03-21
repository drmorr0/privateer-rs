use std::collections::HashMap;
use privateer_custom_derive::ship_component;
use serde::Deserialize;

trait ShipComponent { }

#[derive(Debug, Deserialize)]
pub enum HullClass {
    Light,
    Medium,
    Heavy,
}

#[ship_component]
#[derive(Debug, Deserialize)]
pub struct Hull {
    pub name: String,
    pub class: HullClass,
    pub role: String,
    pub segments: HashMap<String, u8>,
}

impl ShipComponent for Hull {}