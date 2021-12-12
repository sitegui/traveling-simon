use crate::models::*;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub duties: Vec<BoundedTimeWindow>,
    pub service_time: Duration,
    pub must_visit: bool,
    pub can_start_here: bool,
}

impl Site {
    pub fn try_from_json(sites: &IdConverter<SiteId>, input: input::Site) -> Result<Self> {
        Ok(Site {
            id: sites.get(&input.name)?,
            name: input.name,
            duties: input.duties,
            service_time: input.service_time,
            must_visit: input.must_visit,
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
            must_visit: false,
            can_start_here: false,
        }
    }
}
