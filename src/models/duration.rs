#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Duration(i32);

impl Duration {
    pub fn from_s(s: i32) -> Self {
        Duration(s)
    }

    pub fn as_s(self) -> i32 {
        self.0
    }
}
