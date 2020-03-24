use crate::components::*;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Ship {
    pub name: String,
    components: Vec<ComponentType>,
}

impl Ship {
    pub fn new(name: String, hull_template: HullData) -> Ship {
        Ship {
            name,
            components: vec![ComponentType::Hull(hull_template)],
        }
    }

    pub fn add_component(
        &mut self,
        component: ComponentType,
        location: &str,
    ) -> Result<(), String> {
        let next_id = self.components.len();
        match self.segment(location) {
            Some(segment) => {
                if segment.used_slots + slots(&component) > segment.slots {
                    return Err(format!(
                        "The requested component does not fit in {}",
                        location
                    ));
                }
                segment.used_slots += slots(&component);
                segment.component_ids.push(next_id);
                self.components.push(component);
            }
            None => {
                return Err(format!(
                    "The requested location {} does not exist",
                    location
                ))
            }
        }
        Ok(())
    }

    pub fn hull(&self) -> &HullData {
        match &self.components[0] {
            ComponentType::Hull(d) => d,
            _ => panic!("This ship has no hull!"),
        }
    }

    pub fn segment(&mut self, name: &str) -> Option<&mut HullSegment> {
        match &mut self.components[0] {
            ComponentType::Hull(d) => d.segments.get_mut(name),
            _ => panic!("This ship has no hull!"),
        }
    }

    pub fn mass(&self) -> u32 {
        self.components.iter().fold(0, |m, x| m + mass(&x))
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {:?} {})\n",
            self.name,
            self.hull().name,
            self.hull().class,
            self.hull().role
        )?;
        write!(f, "  Mass: {} kg\n", self.mass())?;
        write!(f, "  Installed Components:\n")?;
        for c in self.components.split_first().expect("ohno").1 {
            write!(f, "    - {}\n", name(c))?;
        }
        Ok(())
    }
}
