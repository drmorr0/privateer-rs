use crate::components::*;
use std::fmt;

#[derive(Debug)]
pub struct Ship {
    id: usize,
    pub name: String,
    components: Vec<Box<dyn Component>>,
}

impl Ship {
    pub fn new(name: String, id: usize, hull_template: &Hull) -> Ship {
        Ship {
            name,
            id,
            components: vec![Box::new(hull_template.clone())],
        }
    }

    pub fn add_component(
        &mut self,
        component: &dyn Component,
        location: &str,
    ) -> Result<(), String> {
        let next_id = self.components.len();
        match self.segment(location) {
            Some(segment) => {
                if segment.used_slots + component.slots() > segment.slots {
                    return Err(format!(
                        "The requested component does not fit in {}",
                        location
                    ));
                }
                segment.used_slots += component.slots();
                segment.component_ids.push(next_id);
                self.components.push(component.box_clone());
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

    pub fn hull(&self) -> &Hull {
        match self.components[0].as_any().downcast_ref::<Hull>() {
            Some(h) => h,
            None => panic!("The first component isn't a Hull!"),
        }
    }

    pub fn segment(&mut self, name: &str) -> Option<&mut HullSegment> {
        match self.components[0].as_any_mut().downcast_mut::<Hull>() {
            Some(h) => h.segments.get_mut(name),
            None => panic!("The first component isn't a Hull!"),
        }
    }

    pub fn mass(&self) -> u32 {
        self.components.iter().fold(0, |m, x| m + self.mass())
    }
}

impl fmt::Display for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, {:?} {})\n",
            self.name,
            self.hull().name(),
            self.hull().class,
            self.hull().role
        )?;
        write!(f, "  Mass: {} kg\n", self.mass())?;
        write!(f, "  Installed Components:\n")?;
        for c in self.components.split_first().expect("ohno").1 {
            write!(f, "    - {}\n", c.name())?;
        }
        Ok(())
    }
}
