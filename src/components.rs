use serde::Deserialize;
use std::{
    any::Any,
    fmt,
};

use component_derive::Component;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum ComponentType {
    Hull(usize),
    Engine(usize),
    Weapon(usize),
}

impl ComponentType {
    pub fn to_string(&self) -> String {
        match self {
            ComponentType::Hull(_) => "Hull".to_string(),
            ComponentType::Engine(_) => "Engine".to_string(),
            ComponentType::Weapon(_) => "Weeapon".to_string(),
        }
    }

    pub fn to_plural(&self) -> String {
        match self {
            ComponentType::Hull(_) => "Hulls".to_string(),
            ComponentType::Engine(_) => "Engines".to_string(),
            ComponentType::Weapon(_) => "Weeapons".to_string(),
        }
    }
}

pub fn make_ctype_with_id(ctype: ComponentType, id: usize) -> ComponentType {
    match ctype {
        ComponentType::Hull(_) => ComponentType::Hull(id),
        ComponentType::Engine(_) => ComponentType::Engine(id),
        ComponentType::Weapon(_) => ComponentType::Weapon(id),
    }
}

pub trait Component: std::fmt::Debug + BoxClone {
    // We implement the as_any/as_any_mut functions so we can downcast a component to a specific type
    // https://stackoverflow.com/questions/33687447/how-to-get-a-reference-to-a-concrete-type-from-a-trait-object
    fn as_any(&self) -> &(dyn Any + 'static);
    fn as_any_mut(&mut self) -> &mut (dyn Any + 'static);
    fn ctype(&self) -> ComponentType;
    fn name(&self) -> &str;
    fn mass(&self) -> u32;
    fn slots(&self) -> u8;
}

// This trick helpfully stolen from StackOverflow
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object
//
// Like the selected answer, I have no idea how this works
pub trait BoxClone {
    fn box_clone(&self) -> Box<dyn Component>;
}

impl<T> BoxClone for T
where
    T: 'static + Component + Clone,
{
    fn box_clone(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Box<dyn Component> {
        self.box_clone()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct ComponentData {
    pub ctype: ComponentType,
    pub name: String,
    pub mass: u32,
    pub slots: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub enum HullClass {
    Light,
    Medium,
    Heavy,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HullSegment {
    pub name: String,
    pub armor: u32,
    pub slots: u8,

    #[serde(skip)]
    pub used_slots: u8,

    #[serde(skip)]
    pub component_ids: Vec<usize>,
}

#[derive(Component, Clone, Debug, Deserialize)]
pub struct Hull {
    common: ComponentData,

    pub class: HullClass,
    pub role: String,
    pub segment_list: Vec<HullSegment>,
}

#[derive(Component, Clone, Debug, Deserialize)]
pub struct Computer {
    common: ComponentData,

    pub defense: u32,
    pub offense: u32,
}

#[derive(Component, Clone, Debug, Deserialize)]
pub struct Engine {
    common: ComponentData,

    pub thrust: u32,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n", self.name())?;
        write!(f, "  Mass: {} kg\n", self.mass())?;
        write!(f, "  Slots: {}\n", self.slots())?;
        write!(f, "  Thrust: {} N\n", self.thrust)
    }
}

#[derive(Component, Clone, Debug, Deserialize)]
pub struct Weapon {
    pub common: ComponentData,

    pub accuracy: u32,
    pub damage: (u32, u32, u32),
}
