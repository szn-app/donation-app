use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Account {
    pub id: Uuid,
    pub remarks: Option<String>,
    #[graphql(name = "created_at")]
    pub created_at: OffsetDateTime,
}

impl From<Row> for Account {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            remarks: row.get("remarks"),
            created_at: row.get("created_at"),
        }
    }
} 