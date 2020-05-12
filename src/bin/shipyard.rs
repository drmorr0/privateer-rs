use anyhow::Result as AnyResult;
use privateer::{
    io,
    ship::Ship,
    state_machine,
    template::TemplateStore,
    world::World,
};
use ron::ser::{
    to_string_pretty,
    PrettyConfig,
};
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::Write,
};

fn save_ship(tokens: &[String], world: &World) -> Option<state_machine::ContextAction> {
    let ship_str = to_string_pretty(&world.ships[0], PrettyConfig::default()).unwrap();
    let mut file = File::create(&tokens[0]).unwrap();
    file.write_all(ship_str.as_bytes()).unwrap();
    Some(state_machine::ContextAction::Retry)
}

fn main() -> AnyResult<()> {
    let args: Vec<String> = env::args().collect();

    println!("Welcome to the shipyard.  Here you can build, save, and load custom ship designs.");
    let mut world = World::new();
    world.mk_shop("A Better, Cheaper Shipsmith");
    if args.len() > 1 {
        let s: Ship = io::read_data_file(&args[1]);
        println!("Loaded {} from {}", s.name, args[1]);
        world.ships.push(s);
    } else {
        let response = io::prompt("What would you like to name your ship?");
        world.mk_ship(&response, TemplateStore::hull(0).unwrap().clone());
    };

    let mut commands: HashMap<String, state_machine::CommandFunction> = HashMap::new();
    commands.insert("save".to_string(), save_ship);
    let entry_state = state_machine::shipyard::BuilderRootState::new(0, 0);
    let mut context = state_machine::Context::new(world, entry_state, commands);
    context.run()?;
    Ok(())
}
