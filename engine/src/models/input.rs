use crate::models::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World {
    pub sites: Vec<Site>,
    pub min_start_at: Timestamp,
    pub max_end_at: Option<Timestamp>,
    pub max_tested_extensions: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub name: String,
    pub ride_durations: BTreeMap<String, Duration>,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    pub can_start_here: bool,
    pub must_visit: bool,
}
