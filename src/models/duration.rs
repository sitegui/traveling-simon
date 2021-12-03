use std::ops::{AddAssign, SubAssign};

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
