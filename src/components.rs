use std::collections::HashMap;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum HullClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HullSegment {
    pub armor: u32,
    pub slots: u8,
    pub component_ids: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HullData {
    pub name: String,

    pub class: HullClass,
    pub mass: u32,
    pub role: String,
    pub segments: HashMap<String, HullSegment>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EngineData {
    pub name: String,

    pub mass: u32,
    pub thrust: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeaponData {
    pub name: String, 
    pub accuracy: u32,
    pub damage: (u32, u32, u32),
    pub mass: u32,
}

#[derive(Debug, Deserialize)]
pub enum ComponentType {
    Hull(HullData),
    Engine(EngineData),
    Weapon(WeaponData),
}

pub fn mass(ct: &ComponentType) -> u32 {
    match ct {
        ComponentType::Hull(d) => d.mass,
        ComponentType::Engine(d) => d.mass,
        ComponentType::Weapon(d) => d.mass,
    }
}