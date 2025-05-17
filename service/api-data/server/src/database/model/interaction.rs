use crate::database::model::listing::ItemIntentAction;
use async_graphql::{Enum, SimpleObject};
use async_trait::async_trait;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "pledge_status")]
pub enum PledgeStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "transaction_status")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "message_type")]
pub enum MessageType {
    Text,
    Image,
    Video,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Schedule {
    pub id: i64,
    pub scheduled_for: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Schedule {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            scheduled_for: row.get("scheduled_for"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Pledge {
    pub id: i64,
    pub id_profile: Uuid,
    pub id_item: i64,
    pub intent_action: ItemIntentAction,
    pub message: Option<String>,
    pub status: PledgeStatus,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Pledge {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_profile: row.get("id_profile"),
            id_item: row.get("id_item"),
            intent_action: row.get("intent_action"),
            message: row.get("message"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Transaction {
    pub id: i64,
    pub id_pledge: i64,
    pub status: TransactionStatus,
    pub id_schedule: Option<i64>,
    pub id_location: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
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

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Message {
    pub id: i64,
    pub id_sender: Uuid,
    pub id_transaction: i64,
    pub type_: MessageType,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Message {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_sender: row.get("id_sender"),
            id_transaction: row.get("id_transaction"),
            type_: row.get("type"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Review {
    pub id_transaction: i64,
    pub id_subject_profile: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Review {
    fn from(row: Row) -> Self {
        Self {
            id_transaction: row.get("id_transaction"),
            id_subject_profile: row.get("id_subject_profile"),
            rating: row.get("rating"),
            comment: row.get("comment"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct ScheduleOpportunity {
    pub id: i64,
    pub window_start: OffsetDateTime,
    pub window_end: OffsetDateTime,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for ScheduleOpportunity {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            window_start: row.get("window_start"),
            window_end: row.get("window_end"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}
