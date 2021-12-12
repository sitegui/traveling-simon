use crate::models::*;
use std::cmp::{Ordering, Reverse};
use std::collections::BTreeMap;

/// Represents a set of non-dominated paths
///
/// This means that each path is not strictly better than any other paths
#[derive(Debug, Clone)]
pub struct PathBag {
    items: Vec<PathBagItem>,
    max_items: usize,
}

#[derive(Debug, Clone)]
pub struct PathBagItem {
    pub path: Path,
    // How many paths have greater cost
    pub dominates: i32,
    // How many paths have smaller cost
    pub dominated_by: i32,
}

impl PathBag {
    pub fn new(max_items: usize) -> Self {
        PathBag {
            items: Vec::with_capacity(max_items + 1),
            max_items,
        }
    }

    pub fn into_paths(self) -> impl Iterator<Item = Path> {
        self.items.into_iter().map(|item| item.path)
    }

    pub fn into_sorted_paths(mut self) -> impl Iterator<Item = PathBagItem> {
        // Greater scores first
        self.items.sort_by_key(|item| Reverse(item.score()));
        self.items.into_iter()
    }

    pub fn add(&mut self, new_path: Path) {
        let mut new_item = PathBagItem {
            path: new_path,
            dominates: 0,
            dominated_by: 0,
        };

        let mut worst_index = usize::MAX;
        let mut worst_score = (Reverse(i32::MIN), i32::MAX); // lesser is worse

        // Update dominance counters and find the worst item
        for (i, item) in self.items.iter_mut().enumerate() {
            match new_item.path.cost.partial_cmp(&item.path.cost) {
                Some(Ordering::Less) => {
                    new_item.dominates += 1;
                    item.dominated_by += 1;
                }
                Some(Ordering::Greater) => {
                    new_item.dominated_by += 1;
                    item.dominates += 1;
                }
                _ => {}
            }

            let score = item.score();
            if score < worst_score {
                worst_score = score;
                worst_index = i;
            }
        }

        // Insert new path
        let new_score = new_item.score();
        if new_score < worst_score {
            worst_index = self.items.len();
        }
        self.items.push(new_item);

        // Remove worst item and update dominance counters
        if self.items.len() > self.max_items {
            let worst_path = self.items.swap_remove(worst_index).path;
            for item in &mut self.items {
                match worst_path.cost.partial_cmp(&item.path.cost) {
                    Some(Ordering::Less) => {
                        item.dominated_by -= 1;
                    }
                    Some(Ordering::Greater) => {
                        item.dominates -= 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Return the number of elements by the number of (dominated_by, dominates)
    pub fn count_by_score(&self) -> BTreeMap<(i32, i32), i32> {
        let mut result: BTreeMap<_, i32> = BTreeMap::new();
        for item in &self.items {
            *result
                .entry((item.dominated_by, item.dominates))
                .or_default() += 1;
        }
        result
    }
}

impl PathBagItem {
    fn score(&self) -> (Reverse<i32>, i32) {
        (Reverse(self.dominated_by), self.dominates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use std::collections::BTreeMap;

    fn path_with_cost(a: i32, b: i32) -> Path {
        let mut path = Path::mock();
        path.cost.total_ride = Duration::from(a);
        path.cost.total_time = Duration::from(b);
        path
    }

    fn extract_costs(bag: PathBag) -> Vec<(i32, i32)> {
        bag.into_paths()
            .map(|path| (path.cost.total_ride.as_s(), path.cost.total_time.as_s()))
            .sorted()
            .collect_vec()
    }

    /// Insert items that purely dominate each other
    #[test]
    fn pure_dominance() {
        let mut bag = PathBag::new(3);
        for c in 0..10 {
            bag.add(path_with_cost(c, c));
        }
        assert_eq!(extract_costs(bag), vec![(0, 0), (1, 1), (2, 2)]);

        let mut bag = PathBag::new(3);
        for c in (0..10).rev() {
            bag.add(path_with_cost(c, c));
        }
        assert_eq!(extract_costs(bag), vec![(0, 0), (1, 1), (2, 2)]);
    }

    /// Insert items without dominance
    #[test]
    fn no_dominance() {
        let mut bag = PathBag::new(3);
        for c in 0..10 {
            bag.add(path_with_cost(c, -c));
        }
        // Implementation detail: the least index is dropped every time
        assert_eq!(extract_costs(bag), vec![(1, -1), (2, -2), (9, -9)]);
    }

    #[test]
    fn test() {
        // Enumerates all possibilities with 4 costs represented as points in a 2D integer grid from
        // 0 to 3, with the additional restriction that no two points can share the same row or
        // column. For each case, each point is marked as `true` if it can be dropped if only the
        // three "best" ones were to be kept.
        // Each case is a tuple like `(a, drop_a, b, drop_b, c, drop_c, d, drop_d)` representing the
        // 4 points `(0, a)`, `(1, b)`, `(2, c)` and `(3, d)`.
        // A point is marked as droppable if has the largest number of points that dominate it.
        // In case of a tie, we prefer dropping the ones with the smallest number of dominated
        // points. In case of a tie still, they're all marked as droppable.
        let cases = vec![
            vec![(0, false), (1, false), (2, false), (3, true)],
            vec![(0, false), (1, false), (3, true), (2, true)],
            vec![(0, false), (2, false), (1, false), (3, true)],
            vec![(0, false), (2, false), (3, true), (1, false)],
            vec![(0, false), (3, false), (1, false), (2, true)],
            vec![(0, false), (3, true), (2, true), (1, true)],
            vec![(1, false), (0, false), (2, false), (3, true)],
            vec![(1, false), (0, false), (3, true), (2, true)],
            vec![(1, false), (2, false), (0, false), (3, true)],
            vec![(1, false), (2, false), (3, true), (0, false)],
            vec![(1, false), (3, true), (0, false), (2, true)],
            vec![(1, false), (3, true), (2, true), (0, false)],
            vec![(2, false), (0, false), (1, false), (3, true)],
            vec![(2, false), (0, false), (3, true), (1, false)],
            vec![(2, false), (1, false), (0, false), (3, true)],
            vec![(2, false), (1, false), (3, true), (0, false)],
            vec![(2, false), (3, true), (0, false), (1, true)],
            vec![(2, false), (3, true), (1, false), (0, false)],
            vec![(3, false), (0, false), (1, false), (2, true)],
            vec![(3, false), (0, false), (2, true), (1, true)],
            vec![(3, false), (1, false), (0, false), (2, true)],
            vec![(3, false), (1, false), (2, true), (0, false)],
            vec![(3, false), (2, false), (0, true), (1, true)],
            vec![(3, true), (2, true), (1, true), (0, true)],
        ];

        // For each case, 3 out of the 4 points are added to a bag with maximum size of 3. When the
        // 4th point is added and one of the points is dropped, we check whether that was allowed.
        for points in cases {
            let mut points = points
                .into_iter()
                .enumerate()
                .map(|(x, (y, droppable))| (x as i32, y, droppable))
                .collect_vec();
            check(&points);
            points.rotate_right(1);
            check(&points);
            points.rotate_right(1);
            check(&points);
            points.rotate_right(1);
            check(&points);

            fn check(points: &[(i32, i32, bool)]) {
                let mut bag = PathBag::new(3);
                let mut droppable_by_x = BTreeMap::new();

                for &(x, y, droppable) in points {
                    bag.add(path_with_cost(x, y));
                    droppable_by_x.insert(x, droppable);
                }

                for (x, _) in extract_costs(bag) {
                    droppable_by_x.remove(&x);
                }

                assert_eq!(
                    droppable_by_x.len(),
                    1,
                    "Not exactly one point dropped in {:?}",
                    points
                );
                let dropped = droppable_by_x.into_iter().next().unwrap();
                assert!(
                    dropped.1,
                    "Dropped {}, which is not allowed in {:?}",
                    dropped.0, points
                );
            }
        }
    }
}
