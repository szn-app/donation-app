use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::{
    types::{FromSql, ToSql},
    Row,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "media_type")]
pub enum MediaType {
    #[graphql(name = "document")]
    #[postgres(name = "document")]
    Document,
    #[graphql(name = "image")]
    #[postgres(name = "image")]
    Image,
    #[graphql(name = "video")]
    #[postgres(name = "video")]
    Video,
}

impl Default for MediaType {
    fn default() -> Self {
        MediaType::Image
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Media {
    pub id: i64,
    pub id_item: i64,
    pub caption: Option<String>,
    pub url: String,
    pub variant: MediaType,
    pub created_at: OffsetDateTime,
}

impl From<Row> for Media {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_item: row.get("id_item"),
            caption: row.get("caption"),
            url: row.get("url"),
            variant: row.get("variant"),
            created_at: row.get("created_at"),
        }
    }
}
