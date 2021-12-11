use crate::models::*;
use crate::path_bag::PathBag;

/// Builds a set of interesting paths
pub fn build(world: &World) -> PathBag {
    let mut base_paths = PathBag::new();
    let mut finished_paths = PathBag::new();

    // Add seed paths, based on each desired starting position
    for &start_in in &world.start_in_one_of {
        let path = Path::empty(start_in, world.min_start_at);
        base_paths.add(path);
    }

    while !base_paths.is_empty() {
        log::info!("Build iteration starting from {} paths", base_paths.len());
        base_paths = build_iteration(world, &mut finished_paths, base_paths);
    }

    finished_paths
}

#[derive(Debug)]
struct ExtensionInfo {
    site: SiteId,
    duty: Option<BoundedTimeWindow>,
    earliest_service_start: Timestamp,
}

fn build_iteration(world: &World, finished_paths: &mut PathBag, base_paths: PathBag) -> PathBag {
    let mut new_bag = PathBag::new();

    // For each base path, try all possible extensions
    for base_path in base_paths.into_vec() {
        let something_added = extend_path(world, &base_path, &mut new_bag);

        // If this base path could not produced any good extended path, then it's said to be
        // "finished"
        if !something_added {
            finished_paths.add(base_path);
        }
    }

    new_bag
}

/// Try at most `max_candidates` modifications to the given base path, adding one more stop at the
/// end. Returns `true` if at least one new path was inserted into the resulting bag.
fn extend_path(world: &World, base_path: &Path, sink: &mut PathBag) -> bool {
    let (end_in, end_at) = base_path.end();

    // Collect all possible extensions
    let mut extensions = vec![];
    for site in &world.sites {
        // Don't revisit
        if base_path.visited_sites.contains(&site.id) {
            continue;
        }

        // Ride must exist
        let ride = match world.ride(end_in, site.id) {
            None => continue,
            Some(ride) => ride,
        };

        let earliest_arrival = end_at + ride;
        extensions.push(ExtensionInfo {
            site: site.id,
            duty: None,
            earliest_service_start: earliest_arrival,
        });

        for &duty in &site.duties {
            if earliest_arrival <= duty.start() {
                extensions.push(ExtensionInfo {
                    site: site.id,
                    duty: Some(duty),
                    earliest_service_start: duty.start(),
                });
            }
        }
    }

    // Collect the best candidate sites to extend this path
    let num_before = extensions.len();
    extensions.sort_by_key(|info| info.earliest_service_start);
    let best_extensions = extensions
        .into_iter()
        .take(world.max_tested_extensions.try_into().unwrap());
    log::debug!(
        "Selected {} out of {} valid extensions",
        best_extensions.len(),
        num_before
    );

    let mut changed = false;

    // Try each extension: the vec `new_stops` will be pushed and popped at every iteration
    let mut new_stops = Vec::with_capacity(base_path.stops.len() + 1);
    for stop in &base_path.stops {
        new_stops.push(stop.sketch());
    }
    for extension in best_extensions {
        // Schedule path
        new_stops.push(StopSketch {
            site: extension.site,
            duty: extension.duty,
        });
        let new_path = Path::try_schedule(world, base_path.start_in, &new_stops);
        new_stops.pop();

        if let Some(new_path) = new_path {
            if let Some(max_end_at) = world.max_end_at {
                if new_path.end().1 > max_end_at {
                    log::debug!("Ignore path {} that ends too late", new_path);
                    continue;
                }
            }

            changed |= sink.add(new_path);
        }
    }

    changed
}
