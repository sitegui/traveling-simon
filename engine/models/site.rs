use crate::models::ids::SiteId;
use crate::models::{input, BoundedTimeWindow, Duration, IdConverter};
use anyhow::Result;
use itertools::Itertools;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    pub must_visit: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct SiteAndDuty {
    pub site: SiteId,
    pub duty: Option<BoundedTimeWindow>,
}

impl Site {
    pub fn try_from_json(sites: &IdConverter<SiteId>, input: input::Site) -> Result<Self> {
        Ok(Site {
            id: sites.get(&input.name)?,
            name: input.name,
            duties: input.duties,
            service_time: input.service_time,
            must_visit: input.must_visit,
        })
    }
}
