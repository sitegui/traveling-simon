use crate::models::ids::SiteId;
use crate::models::{BoundedTimeWindow, Duration};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    /// The duration to reach other sites
    pub ride_durations: BTreeMap<SiteId, Duration>,
}

#[derive(Debug, Clone, Copy)]
pub struct SiteAndDuty {
    pub site: SiteId,
    pub duty: Option<BoundedTimeWindow>,
}

impl Site {
    pub fn ride_duration(&self, to: SiteId) -> Option<Duration> {
        self.ride_durations.get(&to).copied()
    }
}
