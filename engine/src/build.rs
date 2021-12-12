use crate::models::*;
use crate::path_bag::PathBag;

/// Builds a set of interesting paths
pub fn build(world: &World) -> PathBag {
    let mut base_paths = PathBag::new(world.max_bag_items);
    let mut finished_paths = PathBag::new(world.max_bag_items);

    // Add seed paths, based on each desired starting position
    for site in &world.sites {
        if site.can_start_here {
            let path = Path::empty(site.id, world.min_start_at);
            base_paths.add(path);
        }
    }

    while !base_paths.is_empty() {
        log::info!(
            "Build iteration starting from {} paths, by score = {:?}",
            base_paths.len(),
            base_paths.count_by_score()
        );
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
    let mut new_bag = PathBag::new(world.max_bag_items);

    // For each base path, try all possible extensions
    for base_path in base_paths.into_paths() {
        extend_path(world, &base_path, &mut new_bag);

        if base_path.visited_sites.is_superset(&world.must_visit) {
            finished_paths.add(base_path);
        }
    }

    new_bag
}

/// Try at most `max_candidates` modifications to the given base path, adding one more stop at the
/// end. Returns `true` if at least one new path was inserted into the resulting bag.
fn extend_path(world: &World, base_path: &Path, sink: &mut PathBag) {
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
    if extensions.is_empty() {
        return;
    } else if extensions.len() <= world.max_tested_extensions {
        log::debug!("Selected {} valid extensions", extensions.len());
    } else {
        log::debug!(
            "Selected {} out of {} valid extensions",
            world.max_tested_extensions,
            extensions.len(),
        );
        extensions.sort_by_key(|info| info.earliest_service_start);
        extensions.truncate(world.max_tested_extensions);
    }

    // Try each extension: the vec `new_stops` will be pushed and popped at every iteration
    let mut new_stops = Vec::with_capacity(base_path.stops.len() + 1);
    for stop in &base_path.stops {
        new_stops.push(stop.sketch());
    }
    for extension in extensions {
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

            sink.add(new_path);
        }
    }
}
