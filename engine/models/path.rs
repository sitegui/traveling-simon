use crate::models::{CappedMax, Duration, SiteAndDuty, SiteId, Stop, Timestamp, World};

#[derive(Debug, Clone)]
pub struct Path {
    start_in: SiteId,
    stops: Vec<Stop>,
}

impl Path {
    pub fn try_schedule(
        world: &World,
        start_in: SiteId,
        start_at: Timestamp,
        stops: Vec<SiteAndDuty>,
    ) -> Option<Self> {
        // Forward schedule and calculate compressions
        let mut prev_site = start_in;
        let mut prev_end = start_at;
        let mut compressions = Vec::with_capacity(stops.len());
        let mut total_compression = Duration::ZERO;
        let mut slack = CappedMax::Max;
        let mut path_stops = Vec::with_capacity(stops.len());

        for stop in &stops {
            let site = &world[stop.site];
            let ride = world.ride_matrix.get(prev_site, stop.site)?;
            let ride_end = prev_end + ride;
            let service_start = match stop.duty {
                Some(duty) if duty.start > ride_end => duty.start,
                _ => ride_end,
            };
            let waiting = service_start - ride_end;
            let compression = slack.min_with(waiting);
            let service_end = service_start + site.service_time;
            let stop_slack = match stop.duty {
                Some(duty) => {
                    if duty.end >= service_end {
                        duty.end - service_end
                    } else {
                        // Unfeasible
                        return None;
                    }
                }
                _ => Duration::ZERO,
            };
            slack = slack.min(CappedMax::Value(stop_slack));
            slack -= compression;

            prev_site = site.id;
            prev_end = service_end;
            compressions.push(compression);
            total_compression += compression;
            path_stops.push(Stop {
                site: stop.site,
                duty: stop.duty,
                is_in_duty: stop.duty.is_some(),
                ride_start: prev_end,
                ride_end,
                service_start,
                service_end,
            });
        }

        // Apply compressions
        for (path_stop, compression) in path_stops.iter_mut().zip(compressions) {
            path_stop.ride_start += total_compression;
            path_stop.ride_end += total_compression;
            total_compression -= compression;
            path_stop.service_start += total_compression;
            path_stop.service_end += total_compression;
        }

        Some(Path {
            start_in,
            stops: path_stops,
        })
    }
}
