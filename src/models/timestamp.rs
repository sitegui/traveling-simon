#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i32);

impl Timestamp {
    pub fn from_s(s: i32) -> Self {
        Timestamp(s)
    }

    pub fn as_s(self) -> i32 {
        self.0
    }
}
