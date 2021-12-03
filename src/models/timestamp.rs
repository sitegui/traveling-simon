use crate::models::Duration;
use anyhow::{ensure, Error};
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
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ensure!(s.len() == 8);
        ensure!(&s[2..3] == ":");
        ensure!(&s[5..6] == ":");
        let h = s[0..2].parse()?;
        let m = s[3..5].parse()?;
        let s = s[6..8].parse()?;
        ensure!(h >= 0 && h < 24);
        ensure!(m >= 0 && m < 60);
        ensure!(s >= 0 && s < 60);
        Ok(Timestamp::from_dhms(0, h, m, s))
    }
}

#[allow(clippy::many_single_char_names)]
impl fmt::Display for Timestamp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (d, h, m, s) = self.as_dhms();
        if d == 0 {
            write!(f, "{:02}:{:02}:{:02}", h, m, s)
        } else {
            write!(f, "{:02}:{:02}:{:02} +{}", h, m, s, d)
        }
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
