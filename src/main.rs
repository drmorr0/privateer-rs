mod ship;
mod template;
mod components;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let hull_templates: Vec<components::HullData> = template::read_template_file("./data/hulls.ron".to_string());
    let ship = ship::Ship::new("The Flying Dutchman".to_string(), hull_templates[0].clone());
    println!("{:?}, {}", ship, ship.mass());
    return Ok(());
}
