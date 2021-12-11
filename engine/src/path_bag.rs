use crate::models::*;
use std::cmp::Ordering;

/// Represents a set of non-dominated paths
///
/// This means that each path is not strictly better than any other paths
#[derive(Debug, Clone)]
pub struct PathBag(Vec<Path>);

impl PathBag {
    pub fn new() -> Self {
        PathBag(vec![])
    }

    pub fn into_vec(self) -> Vec<Path> {
        self.0
    }

    pub fn add(&mut self, path: Path) -> bool {
        for (i, existing) in self.0.iter().enumerate() {
            match path.cost.partial_cmp(&existing.cost) {
                None | Some(Ordering::Equal) => {}
                Some(Ordering::Less) => {
                    // The new path is strictly better than an existing one: replace it and drop any
                    // other dominated path
                    log::debug!(
                        "Accept {} because it's strictly better than {}",
                        path,
                        existing
                    );
                    self.remove_dominated(i + 1, &path);
                    self.0[i] = path;
                    return true;
                }
                Some(Ordering::Greater) => {
                    // The new path is strictly worse than an existing one: ignore it
                    log::debug!(
                        "Reject {} because it's strictly worse than {}",
                        path,
                        existing
                    );
                    return false;
                }
            }
        }

        // The path is neither strictly better nor strictly worse than any other path: just include
        // it
        log::debug!("Accept {} because it's not dominated", path);
        self.0.push(path);
        true
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Remove all existing elements that are dominated by the given path
    fn remove_dominated(&mut self, start: usize, path: &Path) {
        let mut i = start;
        while let Some(existing) = self.0.get(i) {
            if path.cost < existing.cost {
                self.0.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }
}
