use crate::models::*;
use serde::{Deserialize, Serialize};
use std::cmp::{Ordering, Reverse};
use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq, Copy, Serialize, Deserialize)]
pub struct PathCost {
    pub total_ride: Duration,
    pub total_time: Duration,
    pub stops_on_duty: Reverse<i32>,
    pub stops: Reverse<i32>,
}

impl PathCost {
    pub fn new(start_at: Timestamp, stops: &[Stop]) -> Self {
        let mut total_ride = Duration::ZERO;
        let mut stops_on_duty = 0;

        for stop in stops {
            total_ride += stop.ride_end - stop.ride_start;
            if stop.duty.is_some() {
                stops_on_duty += 1;
            }
        }

        let total_time = match stops.last() {
            Some(last) => last.service_end - start_at,
            None => Duration::ZERO,
        };

        PathCost {
            total_ride,
            total_time,
            stops_on_duty: Reverse(stops_on_duty),
            stops: Reverse(stops.len() as i32),
        }
    }
}

impl PartialOrd for PathCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ge = self.total_ride >= other.total_ride
            && self.total_time >= other.total_time
            && self.stops_on_duty >= other.stops_on_duty
            && self.stops >= other.stops;
        let le = self.total_ride <= other.total_ride
            && self.total_time <= other.total_time
            && self.stops_on_duty <= other.stops_on_duty
            && self.stops <= other.stops;
        match (ge, le) {
            (true, true) => Some(Ordering::Equal),
            (true, false) => Some(Ordering::Greater),
            (false, true) => Some(Ordering::Less),
            (false, false) => None,
        }
    }
}

impl fmt::Display for PathCost {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{{total_ride: {}, total_time: {}, stops_on_duty: {}, stops: {}}}",
            self.total_ride, self.total_time, self.stops_on_duty.0, self.stops.0
        )
    }
}
