use crate::models::{Duration, InternalId, SiteId};

#[derive(Debug, Clone)]
pub struct RideMatrix {
    side: usize,
    entries: Vec<Option<Duration>>,
}

impl RideMatrix {
    pub fn empty(side: usize) -> Self {
        RideMatrix {
            side,
            entries: vec![None; side * side],
        }
    }
    
    pub fn get(&self, from: SiteId, to: SiteId) -> Option<Duration> {
        self.entries[from.as_usize() * self.side + to.as_usize()]
    }
    
    pub fn set(&mut self, from: SiteId, to: SiteId, value: Duration) {
        self.entries[from.as_usize() * self.side + to.as_usize()] = Some(value);
    }
}
