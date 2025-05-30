use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "profile_type")]
pub enum ProfileType {
    #[graphql(name = "individual")]
    #[postgres(name = "individual")]
    Individual,
    #[graphql(name = "organization")]
    #[postgres(name = "organization")]
    Organization,
    #[graphql(name = "company")]
    #[postgres(name = "company")]
    Company,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Profile {
    pub id: i64,
    #[graphql(default)]
    pub name: Option<String>,
    #[graphql(default)]
    pub description: Option<String>,
    #[graphql(default)]
    pub variant: Option<ProfileType>,
    pub owner: Uuid,
    pub created_at: OffsetDateTime,
    #[graphql(default)]
    pub updated_at: Option<OffsetDateTime>,
    pub created_by: Uuid,
}

impl From<Row> for Profile {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            variant: row.get("variant"),
            owner: row.get("owner"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
        }
    }
}
