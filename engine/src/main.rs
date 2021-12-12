use crate::models::*;
use anyhow::{Context, Result};
use itertools::Itertools;
use std::io::Read;

mod build;
mod models;
mod parsers;
mod path_bag;

fn main() -> Result<()> {
    env_logger::init();

    // Read world description from stdin
    let mut world_json = String::new();
    std::io::stdin()
        .read_to_string(&mut world_json)
        .context("failed to read stdin")?;
    let world_input: input::World =
        serde_json::from_str(&world_json).context("failed to parse stdin")?;
    let world = World::try_from_json(world_input).context("failed to initialize world")?;
    log::debug!("world = {:?}", world);

    let built = build::build(&world);

    // Print answers
    let answer = built
        .into_sorted_paths()
        .filter(|item| !item.path.stops.is_empty())
        .map(|item| output::Path::new(&world, &item))
        .take(world.max_results)
        .collect_vec();
    let answer_str = serde_json::to_string(&answer).context("failed to encode answer")?;
    println!("{}", answer_str);

    Ok(())
}
