use specs::prelude::*;

pub struct Mass {
    pub kg: u32,
}

pub struct Armor {
    pub value: u32,
}

impl Component for Mass {
    type Storage = DenseVecStorage<Self>;
}
