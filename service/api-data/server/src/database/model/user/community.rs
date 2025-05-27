use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "community_type")]
pub enum CommunityType {
    #[graphql(name = "solo")]
    #[postgres(name = "solo")]
    Solo,
    #[graphql(name = "organized")]
    #[postgres(name = "organized")]
    Organized,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Community {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub type_: CommunityType,
    pub owner: Uuid,
    #[graphql(name = "created_at")]
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub created_by: Uuid,
}

impl From<Row> for Community {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            type_: row.get("type"),
            owner: row.get("owner"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
        }
    }
} 