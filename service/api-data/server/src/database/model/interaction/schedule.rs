use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Schedule {
    pub id: i64,
    #[graphql(name = "scheduled_for")]
    pub scheduled_for: OffsetDateTime,
}

impl From<Row> for Schedule {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            scheduled_for: row.get("scheduled_for"),
        }
    }
} 