use crate::models::{Duration, Site, SiteId, Timestamp};
use std::collections::BTreeSet;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct World {
    pub sites: Vec<Site>,
    pub start_in_one_of: BTreeSet<SiteId>,
    pub min_start_at: Timestamp,
    pub end_in_one_of: BTreeSet<SiteId>,
    pub max_end_at: Option<Timestamp>,
}

impl Index<SiteId> for World {
    type Output = Site;

    fn index(&self, index: SiteId) -> &Self::Output {
        &self.sites[index.as_usize()]
    }
}
