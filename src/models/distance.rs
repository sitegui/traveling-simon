#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Distance(i32);

impl Distance {
    pub fn from_m(m: i32) -> Self {
        Distance(m)
    }

    pub fn as_m(self) -> i32 {
        self.0
    }
}
