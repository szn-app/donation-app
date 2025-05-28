use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Category {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category_parent: Option<i64>,
}

impl From<Row> for Category {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            category_parent: row.get("category_parent"),
        }
    }
} 