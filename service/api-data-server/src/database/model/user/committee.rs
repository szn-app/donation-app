use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "committee_role")]
pub enum CommitteeRole {
    #[graphql(name = "organizer")]
    #[postgres(name = "organizer")]
    Organizer,
    #[graphql(name = "member")]
    #[postgres(name = "member")]
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Committee {
    pub id_profile: i64,
    pub id_community: i64,
    pub member_role: CommitteeRole,
    #[graphql(name = "joined_at")]
    pub joined_at: OffsetDateTime,
}

impl From<Row> for Committee {
    fn from(row: Row) -> Self {
        Self {
            id_profile: row.get("id_profile"),
            id_community: row.get("id_community"),
            member_role: row.get("member_role"),
            joined_at: row.get("joined_at"),
        }
    }
} 