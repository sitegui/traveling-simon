use crate::models::*;
use anyhow::Result;
use std::fs;

mod build;
mod models;
mod parsers;
mod path_bag;

fn main() -> Result<()> {
    env_logger::init();

    let world_input: input::World = serde_json::from_str(&fs::read_to_string("data/world1.json")?)?;

    let world = World::try_from_json(world_input)?;

    eprintln!("world = {:?}", world);

    let built = build::build(&world);
    for path in built.into_vec() {
        println!("path = {}", path);
        println!("\t cost = {}", path.cost);
    }

    Ok(())
}
