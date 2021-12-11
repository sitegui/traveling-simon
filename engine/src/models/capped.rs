use std::ops::SubAssign;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum CappedMax<T> {
    Value(T),
    Max,
}

impl<T: Ord> CappedMax<T> {
    pub fn min_with(self, value: T) -> T {
        match self {
            CappedMax::Max => value,
            CappedMax::Value(self_value) => self_value.min(value),
        }
    }
}

impl<T: SubAssign> SubAssign<T> for CappedMax<T> {
    fn sub_assign(&mut self, rhs: T) {
        if let CappedMax::Value(self_value) = self {
            *self_value -= rhs;
        }
    }
}
