use crate::models::*;
use crate::parsers::parse_timestamp;
use anyhow::{Error, Result};
use nom::combinator::all_consuming;
use nom::Finish;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::ops::{Add, AddAssign, Sub};
use std::str::FromStr;

/// A timestamp represented by the number of seconds since midnight
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i32);

const M: i32 = 60;
const H: i32 = 60 * M;
const D: i32 = 24 * H;

impl Timestamp {
    pub fn from_dhms(d: i32, h: i32, m: i32, s: i32) -> Self {
        Timestamp(D * d + H * h + M * m + s)
    }

    pub fn as_dhms(self) -> (i32, i32, i32, i32) {
        (
            self.0 / D,
            (self.0 / H) % 24,
            (self.0 / M) % 60,
            self.0 % 60,
        )
    }

    #[cfg(test)]
    pub fn mock() -> Self {
        Timestamp(0)
    }
}

#[allow(clippy::many_single_char_names)]
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (d, h, m, s) = self.as_dhms();
        write!(f, "{:02}:{:02}", h, m)?;
        if s != 0 {
            write!(f, ":{:02}", s)?;
        }
        if d != 0 {
            write!(f, " +{}", d)?;
        }
        Ok(())
    }
}

impl Add for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Self) -> Self::Output {
        Timestamp(self.0 + rhs.0)
    }
}

impl Add<Duration> for Timestamp {
    type Output = Timestamp;

    fn add(self, rhs: Duration) -> Self::Output {
        Timestamp(self.0 + rhs.as_s())
    }
}

impl Sub for Timestamp {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration::from_s(self.0 - rhs.0)
    }
}

impl AddAssign<Duration> for Timestamp {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.as_s();
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, timestamp) = all_consuming(parse_timestamp)(s)
            .finish()
            .map_err(|e| Error::msg(e.to_string()))?;
        Ok(timestamp)
    }
}

impl Serialize for Timestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Timestamp {
    fn deserialize<D>(deserializer: D) -> Result<Timestamp, D::Error>
    where
        D: Deserializer<'de>,
    {
        let as_string = String::deserialize(deserializer)?;
        as_string.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl From<i32> for Timestamp {
    fn from(t: i32) -> Self {
        Timestamp(t)
    }
}
