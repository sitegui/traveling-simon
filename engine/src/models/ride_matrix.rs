use crate::models::*;

#[derive(Debug, Clone)]
pub struct RideMatrix {
    side: usize,
    entries: Vec<Option<Duration>>,
}

impl RideMatrix {
    /// Create a matrix with the diagonal as zero and the other entries as `None`
    pub fn new(side: usize) -> Self {
        let mut entries = vec![None; side * side];
        for i in 0..side {
            entries[i * side + i] = Some(Duration::ZERO);
        }
        RideMatrix { side, entries }
    }

    pub fn get(&self, from: SiteId, to: SiteId) -> Option<Duration> {
        self.entries[from.as_usize() * self.side + to.as_usize()]
    }

    pub fn set(&mut self, from: SiteId, to: SiteId, value: Duration) {
        self.entries[from.as_usize() * self.side + to.as_usize()] = Some(value);
    }
}
