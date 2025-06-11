use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "pledge_intent_action")]
pub enum PledgeIntentAction {
    #[graphql(name = "give")]
    #[postgres(name = "give")]
    Give,
    #[graphql(name = "receive")]
    #[postgres(name = "receive")]
    Receive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "pledge_status")]
pub enum PledgeStatus {
    #[graphql(name = "pending")]
    #[postgres(name = "pending")]
    Pending,
    #[graphql(name = "approved")]
    #[postgres(name = "approved")]
    Approved,
    #[graphql(name = "declined")]
    #[postgres(name = "declined")]
    Declined,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Pledge {
    pub id: i64,
    pub id_profile: i64,
    pub id_item: i64,
    pub intent_action: PledgeIntentAction,
    #[graphql(default)]
    pub message: Option<String>,
    pub status: PledgeStatus,
    #[graphql(name = "pledged_at")]
    pub pledged_at: OffsetDateTime,
    #[graphql(default)]
    pub updated_at: Option<OffsetDateTime>,
    #[graphql(default)]
    pub updated_by: Option<Uuid>,
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
            pledged_at: row.get("pledged_at"),
            updated_at: row.get("updated_at"),
            updated_by: row.get("updated_by"),
        }
    }
} 