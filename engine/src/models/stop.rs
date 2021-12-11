use crate::models::*;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct StopSketch {
    pub site: SiteId,
    pub duty: Option<BoundedTimeWindow>,
}

#[derive(Debug, Clone)]
pub struct Stop {
    pub site: SiteId,
    pub duty: Option<BoundedTimeWindow>,
    pub ride_start: Timestamp,
    pub ride_end: Timestamp,
    pub service_start: Timestamp,
    pub service_end: Timestamp,
}

impl Stop {
    pub fn sketch(&self) -> StopSketch {
        StopSketch {
            site: self.site,
            duty: self.duty,
        }
    }
}

impl fmt::Display for Stop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.duty {
            None => {
                write!(
                    f,
                    "{} @ {} -> {}",
                    self.site, self.ride_end, self.service_end
                )
            }
            Some(duty) => {
                write!(
                    f,
                    "{}(duty {} - {}) @ {} -> {}",
                    self.site,
                    duty.start(),
                    duty.end(),
                    self.ride_end,
                    self.service_end
                )
            }
        }
    }
}
