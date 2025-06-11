use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Publish {
    pub id_item: i64,
    pub id_collection: i64,
    pub note: Option<String>,
    pub position: i32,
    pub added_by: Option<Uuid>,
    #[graphql(name = "posted_on")]
    pub posted_on: OffsetDateTime,
}

impl From<Row> for Publish {
    fn from(row: Row) -> Self {
        Self {
            id_item: row.get("id_item"),
            id_collection: row.get("id_collection"),
            note: row.get("note"),
            position: row.get("position"),
            added_by: row.get("added_by"),
            posted_on: row.get("posted_on"),
        }
    }
} 