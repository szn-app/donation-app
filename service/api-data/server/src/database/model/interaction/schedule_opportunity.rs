use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct ScheduleOpportunity {
    pub id: i64,
    pub window_start: Option<OffsetDateTime>,
    pub window_end: Option<OffsetDateTime>,
}

impl From<Row> for ScheduleOpportunity {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            window_start: row.get("window_start"),
            window_end: row.get("window_end"),
        }
    }
} 