use crate::models;
use crate::models::*;
use crate::path_bag::PathBagItem;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Path {
    pub start_in: String,
    pub start_at: Timestamp,
    pub stops: Vec<Stop>,
    pub cost: PathCost,
    pub is_dominated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stop {
    pub site: String,
    pub duty: Option<BoundedTimeWindow>,
    pub ride_start: Timestamp,
    pub ride_end: Timestamp,
    pub service_start: Timestamp,
    pub service_end: Timestamp,
}

impl Path {
    pub fn new(world: &World, item: &PathBagItem) -> Self {
        let path = &item.path;
        Path {
            start_in: world[path.start_in].name.clone(),
            start_at: path.start_at,
            stops: path
                .stops
                .iter()
                .map(|stop| Stop::new(world, stop))
                .collect(),
            cost: path.cost,
            is_dominated: item.dominated_by > 0,
        }
    }
}

impl Stop {
    pub fn new(world: &World, stop: &models::Stop) -> Self {
        Stop {
            site: world[stop.site].name.clone(),
            duty: stop.duty,
            ride_start: stop.ride_start,
            ride_end: stop.ride_end,
            service_start: stop.service_start,
            service_end: stop.service_end,
        }
    }
}
