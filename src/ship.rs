use crate::{
    components::*,
    util::{
        enumiter,
        enumiter_mut,
    },
};
use anyhow::Result as AnyResult;
use std::{
    fmt,
    iter::Enumerate,
    slice::{
        Iter,
        IterMut,
    },
};

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

    pub fn add_component(&mut self, component: &dyn Component, id: usize) -> AnyResult<u8, u8> {
        let next_id = self.components.len();
        let segment = &mut self.hull_mut().segment_list[id];
        let slots_remaining = segment.slots - segment.used_slots;
        if component.slots() > slots_remaining {
            return Err(slots_remaining);
        }
        segment.used_slots += component.slots();
        segment.component_ids.push(next_id);
        self.components.push(component.box_clone());
        Ok(slots_remaining - component.slots())
    }

    pub fn hull(&self) -> &Hull {
        match self.components[0].as_any().downcast_ref::<Hull>() {
            Some(h) => h,
            None => panic!("The first component isn't a Hull!"),
        }
    }

    pub fn hull_mut(&mut self) -> &mut Hull {
        match self.components[0].as_any_mut().downcast_mut::<Hull>() {
            Some(h) => h,
            None => panic!("The first component isn't a Hull!"),
        }
    }

    pub fn segment(&self, id: usize) -> &HullSegment {
        &self.hull().segment_list[id]
    }

    pub fn segments(&self) -> Enumerate<Iter<'_, HullSegment>> {
        enumiter(&self.hull().segment_list)
    }

    pub fn segments_mut(&mut self) -> Enumerate<IterMut<'_, HullSegment>> {
        enumiter_mut(&mut self.hull_mut().segment_list)
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
