mod ship;
mod template;
mod components;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let hull_templates: Vec<components::Hull> = template::read_template_file("./data/hulls.ron".to_string());
    println!("{:?}", hull_templates);
    return Ok(());
}
