use crate::models::{input, Duration, IdConverter, InternalId, Site, SiteId, Timestamp};
use anyhow::Result;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct World {
    pub sites: Vec<Site>,
    pub start_in_one_of: BTreeSet<SiteId>,
    pub min_start_at: Timestamp,
    pub end_in_one_of: BTreeSet<SiteId>,
    pub max_end_at: Option<Timestamp>,
}

impl Index<SiteId> for World {
    type Output = Site;

    fn index(&self, index: SiteId) -> &Self::Output {
        &self.sites[index.as_usize()]
    }
}

impl World {
    pub fn try_from_json(input: input::World) -> Result<Self> {
        let sites = IdConverter::new(input.sites.iter().map(|site| site.name.clone()));

        Ok(World {
            sites: input
                .sites
                .into_iter()
                .enumerate()
                .map(|(i, site)| Site::try_from_json(&sites, site))
                .try_collect()?,
            start_in_one_of: input
                .start_in_one_of
                .into_iter()
                .map(|el| sites.get(&el))
                .try_collect()?,
            min_start_at: input.min_start_at,
            end_in_one_of: input
                .end_in_one_of
                .into_iter()
                .map(|el| sites.get(&el))
                .try_collect()?,
            max_end_at: input.max_end_at,
        })
    }
}
