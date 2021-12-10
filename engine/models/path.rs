use crate::models::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Path {
    start_in: SiteId,
    stops: Vec<Stop>,
}

impl Path {
    pub fn try_schedule(world: &World, start_in: SiteId, stops: &[SiteAndDuty]) -> Option<Self> {
        if stops.is_empty() {
            return None;
        }

        // Forward schedule and calculate compressions
        let mut prev_site = start_in;
        let mut prev_end = world.min_start_at;
        let mut compressions = Vec::with_capacity(stops.len());
        let mut total_compression = Duration::ZERO;
        let mut slack = CappedMax::Max;
        let mut path_stops = Vec::with_capacity(stops.len());

        for stop in stops {
            let site = &world[stop.site];
            let ride = world.ride_matrix.get(prev_site, stop.site)?;
            let ride_start = prev_end;
            let ride_end = ride_start + ride;
            let service_start = match stop.duty {
                Some(duty) if duty.start() > ride_end => duty.start(),
                _ => ride_end,
            };
            let waiting = service_start - ride_end;
            let compression = slack.min_with(waiting);
            let service_end = service_start + site.service_time;
            let stop_slack = match stop.duty {
                Some(duty) => {
                    if duty.end() >= service_end {
                        duty.end() - service_end
                    } else {
                        // Unfeasible
                        return None;
                    }
                }
                _ => Duration::ZERO,
            };
            slack -= compression;
            slack = slack.min(CappedMax::Value(stop_slack));

            prev_site = site.id;
            prev_end = service_end;
            compressions.push(compression);
            total_compression += compression;
            path_stops.push(Stop {
                site: stop.site,
                duty: stop.duty,
                ride_start,
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

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} @ {}", self.start_in, self.stops[0].ride_start)?;
        for stop in &self.stops {
            write!(f, "; {}", stop)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schedule() {
        let site0 = Site::mock();
        let mut site1 = Site::mock();
        site1.service_time = Duration::from(1);
        let mut site2 = Site::mock();
        site2.service_time = Duration::from(2);

        let mut world = World::mock(vec![site0, site1, site2]);
        world.min_start_at = Timestamp::from(0);
        world
            .ride_matrix
            .set(SiteId::from(0), SiteId::from(1), Duration::from(10));
        world
            .ride_matrix
            .set(SiteId::from(1), SiteId::from(2), Duration::from(20));

        let path = Path::try_schedule(
            &world,
            SiteId::from(0),
            &[
                SiteAndDuty {
                    site: SiteId::from(1),
                    duty: Some(BoundedTimeWindow::from((15, 18))),
                },
                SiteAndDuty {
                    site: SiteId::from(2),
                    duty: Some(BoundedTimeWindow::from((39, 50))),
                },
            ],
        )
        .unwrap();

        assert_eq!(path.stops[0].ride_start, Timestamp::from(7));
        assert_eq!(path.stops[0].ride_end, Timestamp::from(17));
        assert_eq!(path.stops[0].service_start, Timestamp::from(17));
        assert_eq!(path.stops[0].service_end, Timestamp::from(18));
        assert_eq!(path.stops[1].ride_start, Timestamp::from(18));
        assert_eq!(path.stops[1].ride_end, Timestamp::from(38));
        assert_eq!(path.stops[1].service_start, Timestamp::from(39));
        assert_eq!(path.stops[1].service_end, Timestamp::from(41));
    }
}
