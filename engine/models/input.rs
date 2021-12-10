use crate::models::{BoundedTimeWindow, Duration, Timestamp};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub sites: Vec<Site>,
    pub start_in_one_of: BTreeSet<String>,
    pub min_start_at: Timestamp,
    pub end_in_one_of: BTreeSet<String>,
    pub max_end_at: Option<Timestamp>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    pub ride_durations: BTreeMap<String, Duration>,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    pub must_visit: bool,
}
