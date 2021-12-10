use crate::models::*;
use anyhow::Result;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct World {
    pub sites: Vec<Site>,
    pub start_in_one_of: BTreeSet<SiteId>,
    pub min_start_at: Timestamp,
    pub end_in_one_of: BTreeSet<SiteId>,
    pub max_end_at: Option<Timestamp>,
    pub ride_matrix: RideMatrix,
}

impl Index<SiteId> for World {
    type Output = Site;

    fn index(&self, index: SiteId) -> &Self::Output {
        &self.sites[index.as_usize()]
    }
}

impl World {
    pub fn try_from_json(input: input::World) -> Result<Self> {
        let sites = IdConverter::new(input.sites.iter().map(|site| site.name.clone()))?;

        let mut ride_matrix = RideMatrix::new(input.sites.len());
        for from_site in &input.sites {
            let from_site_id = sites.get(&from_site.name)?;
            for (to_site, &duration) in &from_site.ride_durations {
                let to_site_id = sites.get(to_site)?;
                ride_matrix.set(from_site_id, to_site_id, duration);
            }
        }

        Ok(World {
            sites: input
                .sites
                .into_iter()
                .map(|site| Site::try_from_json(&sites, site))
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
            ride_matrix,
        })
    }

    #[cfg(test)]
    pub fn mock(mut sites: Vec<Site>) -> Self {
        for (i, site) in sites.iter_mut().enumerate() {
            site.id = SiteId::from_usize(i);
            site.name = i.to_string();
        }
        let ride_matrix = RideMatrix::new(sites.len());
        World {
            sites,
            start_in_one_of: Default::default(),
            min_start_at: Timestamp::mock(),
            end_in_one_of: Default::default(),
            max_end_at: None,
            ride_matrix,
        }
    }
}
