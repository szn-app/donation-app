use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "transaction_status")]
pub enum TransactionStatus {
    #[graphql(name = "inprogress")]
    #[postgres(name = "inprogress")]
    InProgress,
    #[graphql(name = "completed")]
    #[postgres(name = "completed")]
    Completed,
    #[graphql(name = "cancelled")]
    #[postgres(name = "cancelled")]
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Transaction {
    pub id: i64,
    pub id_pledge: i64,
    pub status: TransactionStatus,
    pub id_schedule: Option<i64>,
    pub id_location: Option<i64>,
    #[graphql(name = "created_at")]
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Row> for Transaction {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_pledge: row.get("id_pledge"),
            status: row.get("status"),
            id_schedule: row.get("id_schedule"),
            id_location: row.get("id_location"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
