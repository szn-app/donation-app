use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "message_type")]
pub enum MessageType {
    #[graphql(name = "text")]
    Text,
    #[graphql(name = "schedule_opportunity")]
    ScheduleOpportunity,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Message {
    pub id: i64,
    pub id_sender: Option<i64>,
    pub id_transaction: i64,
    pub type_: Option<MessageType>,
    pub content: String,
    #[graphql(name = "sent_at")]
    pub sent_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Row> for Message {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_sender: row.get("id_sender"),
            id_transaction: row.get("id_transaction"),
            type_: row.get("type"),
            content: row.get("content"),
            sent_at: row.get("sent_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 