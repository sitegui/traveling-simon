use crate::models::Timestamp;

#[derive(Debug, Clone)]
pub struct TimeWindow {
    start: Option<Timestamp>,
    end: Option<Timestamp>,
}
