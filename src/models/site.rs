use crate::models::ids::SiteId;
use crate::models::TimeWindow;

#[derive(Debug, Clone)]
pub struct Site {
    pub id: SiteId,
    pub name: String,
    pub duties: Vec<TimeWindow>,
}
