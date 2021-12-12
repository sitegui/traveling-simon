use crate::models::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    pub visit: Visit,
    pub can_start_here: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Visit {
    Always,
    Maybe,
    Never,
}

impl Site {
    pub fn try_from_json(sites: &IdConverter<SiteId>, input: input::Site) -> Result<Self> {
        Ok(Site {
            id: sites.get(&input.name)?,
            name: input.name,
            duties: input.duties,
            service_time: input.service_time,
            visit: input.visit,
            can_start_here: input.can_start_here,
        })
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Site {
            id: SiteId::from_usize(0),
            name: String::new(),
            duties: vec![],
            service_time: Duration::ZERO,
            visit: Visit::Always,
            can_start_here: false,
        }
    }
}
