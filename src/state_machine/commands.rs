use crate::world::World;
use anyhow::Result as AnyResult;
use ron::ser::{
    to_string_pretty,
    PrettyConfig,
};
use std::{
    fs::File,
    io::Write,
};

pub fn save_state(filename: &str, world: &World) -> AnyResult<()> {
    let ship_str = to_string_pretty(world, PrettyConfig::default())?;
    let mut file = File::create(filename)?;
    file.write_all(ship_str.as_bytes())?;
    Ok(())
}
