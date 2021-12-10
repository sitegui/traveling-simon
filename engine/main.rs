use crate::models::{input, World};
use anyhow::Result;
use std::fs;

mod models;
mod parsers;

fn main() -> Result<()> {
    let world_input: input::World = serde_json::from_str(&fs::read_to_string("data/world1.json")?)?;

    let world = World::try_from_json(world_input)?;

    eprintln!("world = {:?}", world);

    Ok(())
}
