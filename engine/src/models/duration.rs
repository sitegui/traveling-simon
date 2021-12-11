use crate::parsers::parse_duration;
use anyhow::{ensure, Error, Result};
use nom::Finish;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(i32);

impl Duration {
    pub const ZERO: Duration = Duration(0);

    pub fn from_s(s: i32) -> Self {
        Duration(s)
    }

    pub fn as_s(self) -> i32 {
        self.0
    }
}

impl SubAssign for Duration {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl AddAssign for Duration {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.as_s() == 0 {
            write!(f, "0s")
        } else {
            let s = self.as_s() % 60;
            let m = (self.as_s() / 60) % 60;
            let h = self.as_s() / 3600;
            if h != 0 {
                write!(f, "{}h", h)?;
            }
            if m != 0 {
                write!(f, "{}m", m)?;
            }
            if s != 0 {
                write!(f, "{}s", s)?;
            }
            Ok(())
        }
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (input, duration) = parse_duration(s)
            .finish()
            .map_err(|e| Error::msg(e.to_string()))?;
        ensure!(input.is_empty());
        Ok(duration)
    }
}

impl Serialize for Duration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Duration {
    fn deserialize<D>(deserializer: D) -> Result<Duration, D::Error>
    where
        D: Deserializer<'de>,
    {
        let as_string = String::deserialize(deserializer)?;
        as_string.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl From<i32> for Duration {
    fn from(t: i32) -> Self {
        Duration(t)
    }
}
