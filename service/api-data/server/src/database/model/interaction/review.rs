use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Review {
    pub id_transaction: i64,
    pub id_subject_profile: i64,
    pub reviewer: i64,
    pub comment: Option<String>,
    pub score: i16,
    pub created_at: OffsetDateTime,
}

impl From<Row> for Review {
    fn from(row: Row) -> Self {
        Self {
            id_transaction: row.get("id_transaction"),
            id_subject_profile: row.get("id_subject_profile"),
            reviewer: row.get("reviewer"),
            comment: row.get("comment"),
            score: row.get("score"),
            created_at: row.get("created_at"),
        }
    }
} 