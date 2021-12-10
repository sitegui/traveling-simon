use crate::models::*;
use anyhow::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A non-empty time window, bounded in both sides
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy)]
pub enum TimeWindow {
    Bounded(BoundedTimeWindow),
    LeftBounded(LeftBoundedTimeWindow),
    RightBounded(RightBoundedTimeWindow),
    Unbounded(UnboundedTimeWindow),
}

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

impl fmt::Display for TimeWindow {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TimeWindow::Bounded(tw) => tw.fmt(f),
            TimeWindow::LeftBounded(tw) => tw.fmt(f),
            TimeWindow::RightBounded(tw) => tw.fmt(f),
            TimeWindow::Unbounded(tw) => tw.fmt(f),
        }
    }
}

#[cfg(test)]
impl From<(i32, i32)> for BoundedTimeWindow {
    fn from((start, end): (i32, i32)) -> Self {
        BoundedTimeWindow::try_new(start.into(), end.into()).unwrap()
    }
}
