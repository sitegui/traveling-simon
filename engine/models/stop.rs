use crate::models::{BoundedTimeWindow, Site, SiteId, TimeWindow, Timestamp};

#[derive(Debug, Clone)]
pub struct Stop {
    pub site: SiteId,
    pub duty: Option<BoundedTimeWindow>,
    pub is_in_duty: bool,
    pub ride_start: Timestamp,
    pub ride_end: Timestamp,
    pub service_start: Timestamp,
    pub service_end: Timestamp,
}
