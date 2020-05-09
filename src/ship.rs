use crate::{
    components::*,
    util::{
        enumiter,
        enumiter_mut,
    },
};
use anyhow::Result as AnyResult;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    collections::BTreeMap,
    fmt,
    iter::Enumerate,
    slice::{
        Iter,
        IterMut,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Ship {
    id: usize,
    pub name: String,
    pub components: BTreeMap<usize, Box<dyn Component>>,
    next_component_id: usize,
}

impl Ship {
    pub fn new(name: String, id: usize, hull_template: &Hull) -> Ship {
        let mut s = Ship {
            name,
            id,
            components: BTreeMap::new(),
            next_component_id: 0,
        };
        s.components.insert(0, Box::new(hull_template.clone()));
        s
    }

    fn new_component_id(&mut self) -> usize {
        self.next_component_id += 1;
        self.next_component_id
    }

    pub fn add_component(&mut self, component: &dyn Component, segment_id: usize) -> AnyResult<u8, u8> {
        let component_id = self.new_component_id();
        let segment = &mut self.hull_mut().segment_list[segment_id];
        let slots_remaining = segment.slots - segment.used_slots;
        if component.slots() > slots_remaining {
            return Err(slots_remaining);
        }
        segment.used_slots += component.slots();
        segment.component_ids.push(component_id);
        self.components.insert(component_id, component.box_clone());
        Ok(slots_remaining - component.slots())
    }

    pub fn remove_component(&mut self, component_id: usize) -> Box<dyn Component> {
        let c = self.components.remove(&component_id).unwrap();
        for (_, segment) in self.segments_mut() {
            match segment.component_ids.iter().position(|&id| id == component_id) {
                Some(i) => {
                    segment.used_slots -= c.slots();
                    segment.component_ids.swap_remove(i);
                    break;
                },
                None => (),
            }
        }
        c
    }

    pub fn hull(&self) -> &Hull {
        match self.components.get(&0).unwrap().as_any().downcast_ref::<Hull>() {
            Some(h) => h,
            None => panic!("The first component isn't a Hull!"),
        }
    }

    pub fn hull_mut(&mut self) -> &mut Hull {
        match self.components.get_mut(&0).unwrap().as_any_mut().downcast_mut::<Hull>() {
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
        self.components.iter().fold(0, |m, (_, c)| m + c.mass())
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
        for (_, segment) in self.segments() {
            println!(
                "    [{}] ({}/{} slots used)",
                segment.name, segment.used_slots, segment.slots
            );
            for id in segment.component_ids.iter() {
                let c = &self.components[id];
                println!("     - {} ({} slots)", c.name(), c.slots());
            }
        }
        Ok(())
    }
}
