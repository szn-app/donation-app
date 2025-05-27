use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "collection_visibility")]
pub enum CollectionVisibility {
    #[graphql(name = "public")]
    #[postgres(name = "public")]
    Public,
    #[graphql(name = "restricted")]
    #[postgres(name = "restricted")]
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "collection_type")]
pub enum CollectionType {
    #[graphql(name = "featured")]
    #[postgres(name = "featured")]
    Featured,
    #[graphql(name = "regular")]
    #[postgres(name = "regular")]
    Regular,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Collection {
    pub id: i64,
    pub id_community: Option<i64>,
    pub title: Option<String>,
    pub visibility: CollectionVisibility,
    pub type_: Option<CollectionType>,
    pub position: i32,
    #[graphql(name = "created_at")]
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
}

impl From<Row> for Collection {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_community: row.get("id_community"),
            title: row.get("title"),
            visibility: row.get("visibility"),
            type_: row.get("type"),
            position: row.get("position"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
} 