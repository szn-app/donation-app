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
#[postgres(name = "item_type")]
pub enum ItemType {
    InKind,
    Inquiry,
    Monetary,
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_intent_action")]
pub enum ItemIntentAction {
    Request,
    Offer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_status")]
pub enum ItemStatus {
    Draft,
    Active,
    Disabled,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_condition")]
pub enum ItemCondition {
    BrandNew,
    PreOwnedBarelyUsed,
    PreOwnedUsable,
    PreOwnedDamaged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "collection_visibility")]
pub enum CollectionVisibility {
    Public,
    Restricted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "collection_type")]
pub enum CollectionType {
    Featured,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "media_type")]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Category {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub category_parent: Option<i64>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Category {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            category_parent: row.get("category_parent"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Location {
    pub id: i64,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub district: Option<String>,
    pub country: String,
    pub geom: Option<String>,
    pub entrance_note: Option<String>,
    pub created_at: OffsetDateTime
}

impl From<Row> for Location {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            address_line1: row.get("address_line1"),
            address_line2: row.get("address_line2"),
            city: row.get("city"),
            state: row.get("state"),
            district: row.get("district"),
            country: row.get("country"),
            geom: row.get("geom"),
            entrance_note: row.get("entrance_note"),
            created_at: row.get("created_at")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Item {
    pub id: i64,
    pub type_: ItemType,
    pub intent_action: ItemIntentAction,
    pub status: ItemStatus,
    pub title: Option<String>,
    pub description: Option<String>,
    pub category: Option<i64>,
    pub condition: ItemCondition,
    pub location: Option<i64>,
    pub views_count: i64,
    pub is_reported: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub created_by: Option<Uuid>
}

impl From<Row> for Item {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            type_: row.get("type"),
            intent_action: row.get("intent_action"),
            status: row.get("status"),
            title: row.get("title"),
            description: row.get("description"),
            category: row.get("category"),
            condition: row.get("condition"),
            location: row.get("location"),
            views_count: row.get("views_count"),
            is_reported: row.get("is_reported"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by")
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Collection {
    pub id: i64,
    pub id_community: i64,
    pub title: String,
    pub visibility: CollectionVisibility,
    pub type_: CollectionType,
    pub position: i32,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
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

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Media {
    pub id: i64,
    pub id_item: i64,
    pub caption: Option<String>,
    pub url: String,
    pub type_: MediaType,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl From<Row> for Media {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            id_item: row.get("id_item"),
            caption: row.get("caption"),
            url: row.get("url"),
            type_: row.get("type"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Publish {
    pub id_item: i64,
    pub id_collection: i64,
    pub note: Option<String>,
    pub position: i32,
    pub added_by: Option<Uuid>,
    pub posted_on: OffsetDateTime
}

impl From<Row> for Publish {
    fn from(row: Row) -> Self {
        Self {
            id_item: row.get("id_item"),
            id_collection: row.get("id_collection"),
            note: row.get("note"),
            position: row.get("position"),
            added_by: row.get("added_by"),
            posted_on: row.get("posted_on")
        }
    }
}
