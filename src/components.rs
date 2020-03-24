use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

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

    #[serde(skip)]
    pub used_slots: u8,

    #[serde(skip)]
    pub component_ids: Vec<usize>,
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
pub struct ComputerData {
    pub name: String,

    pub defense: u32,
    pub offense: u32,
    pub mass: u32,
    pub slots: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EngineData {
    pub name: String,

    pub mass: u32,
    pub slots: u8,
    pub thrust: u32,
}

impl fmt::Display for EngineData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n", self.name)?;
        write!(f, "  Mass: {} kg\n", self.mass)?;
        write!(f, "  Slots: {}\n", self.slots)?;
        write!(f, "  Thrust: {} N\n", self.slots)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WeaponData {
    pub name: String,

    pub accuracy: u32,
    pub damage: (u32, u32, u32),
    pub mass: u32,
    pub slots: u8,
}

#[derive(Debug, Deserialize)]
pub enum ComponentType {
    Hull(HullData),
    Computer(ComputerData),
    Engine(EngineData),
    Weapon(WeaponData),
}

pub fn name(ct: &ComponentType) -> &str {
    match ct {
        ComponentType::Hull(d) => &d.name,
        ComponentType::Computer(d) => &d.name,
        ComponentType::Engine(d) => &d.name,
        ComponentType::Weapon(d) => &d.name,
    }  
}

pub fn mass(ct: &ComponentType) -> u32 {
    match ct {
        ComponentType::Hull(d) => d.mass,
        ComponentType::Computer(d) => d.mass,
        ComponentType::Engine(d) => d.mass,
        ComponentType::Weapon(d) => d.mass,
    }
}

pub fn slots(ct: &ComponentType) -> u8 {
    match ct {
        ComponentType::Hull(_) => panic!("Hulls don't use slots"),
        ComponentType::Computer(d) => d.slots,
        ComponentType::Engine(d) => d.slots,
        ComponentType::Weapon(d) => d.slots,
    }
}
