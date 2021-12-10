use anyhow::{Context, Result};
use std::collections::BTreeMap;

macro_rules! derive_id {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub struct $name(u8);

        impl InternalId for $name {
            fn as_usize(self) -> usize {
                self.0 as usize
            }

            fn from_usize(i: usize) -> Self {
                $name(u8::try_from(i).unwrap())
            }
        }
    };
}

derive_id! {SiteId}

pub trait InternalId: Copy {
    fn as_usize(self) -> usize;

    fn from_usize(i: usize) -> Self;
}

#[derive(Debug, Clone)]
pub struct IdConverter<ID>(BTreeMap<String, ID>);

impl<ID: InternalId> IdConverter<ID> {
    pub fn new(names: impl IntoIterator<Item = String>) -> Self {
        IdConverter(
            names
                .into_iter()
                .enumerate()
                .map(|(i, name)| (name, ID::from_usize(i)))
                .collect(),
        )
    }

    pub fn get(&self, name: &str) -> Result<ID> {
        self.0
            .get(name)
            .copied()
            .with_context(|| format!("Could not find entity with name {}", name))
    }
}
