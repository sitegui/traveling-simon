use crate::models::*;
use anyhow::{ensure, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;

/// A non-empty time window, bounded in both sides
#[derive(Debug, Clone, Copy, Serialize)]
pub struct BoundedTimeWindow {
    /// Inclusive
    start: Timestamp,
    /// Inclusive
    end: Timestamp,
}

#[derive(Debug, Clone, Copy)]
pub struct LeftBoundedTimeWindow {
    pub start: Timestamp,
}

#[derive(Debug, Clone, Copy)]
pub struct RightBoundedTimeWindow {
    pub end: Timestamp,
}

#[derive(Debug, Clone, Copy)]
pub struct UnboundedTimeWindow;

impl BoundedTimeWindow {
    pub fn try_new(start: Timestamp, end: Timestamp) -> Result<Self> {
        ensure!(end > start);
        Ok(BoundedTimeWindow { start, end })
    }

    pub fn start(&self) -> Timestamp {
        self.start
    }

    pub fn end(&self) -> Timestamp {
        self.end
    }
}

impl fmt::Display for BoundedTimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

impl fmt::Display for LeftBoundedTimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, ∞]", self.start)
    }
}

impl fmt::Display for RightBoundedTimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[-∞, {}]", self.end)
    }
}

impl fmt::Display for UnboundedTimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[-∞, ∞]")
    }
}

impl<'de> Deserialize<'de> for BoundedTimeWindow {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            start: Timestamp,
            end: Timestamp,
        }

        let helper = Helper::deserialize(deserializer)?;
        BoundedTimeWindow::try_new(helper.start, helper.end).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl From<(i32, i32)> for BoundedTimeWindow {
    fn from((start, end): (i32, i32)) -> Self {
        BoundedTimeWindow::try_new(start.into(), end.into()).unwrap()
    }
}
