use async_graphql::{Enum, SimpleObject};
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
    #[graphql(name = "inkind")]
    #[postgres(name = "inkind")]
    InKind,
    #[graphql(name = "inquiry")]
    #[postgres(name = "inquiry")]
    Inquiry,
    #[graphql(name = "monetary")]
    #[postgres(name = "monetary")]
    Monetary,
    #[graphql(name = "service")]
    #[postgres(name = "service")]
    Service,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_intent_action")]
pub enum ItemIntentAction {
    #[graphql(name = "request")]
    #[postgres(name = "request")]
    Request,
    #[graphql(name = "offer")]
    #[postgres(name = "offer")]
    Offer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_status")]
pub enum ItemStatus {
    #[graphql(name = "draft")]
    #[postgres(name = "draft")]
    Draft,
    #[graphql(name = "active")]
    #[postgres(name = "active")]
    Active,
    #[graphql(name = "disabled")]
    #[postgres(name = "disabled")]
    Disabled,
    #[graphql(name = "archived")]
    #[postgres(name = "archived")]
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "item_condition")]
pub enum ItemCondition {
    #[graphql(name = "brand_new")]
    #[postgres(name = "brand_new")]
    BrandNew,
    #[graphql(name = "pre_owned_barely_used")]
    #[postgres(name = "pre_owned_barely_used")]
    PreOwnedBarelyUsed,
    #[graphql(name = "pre_owned_usable")]
    #[postgres(name = "pre_owned_usable")]
    PreOwnedUsable,
    #[graphql(name = "pre_owned_damaged")]
    #[postgres(name = "pre_owned_damaged")]
    PreOwnedDamaged,
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
    #[graphql(name = "created_at")]
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub created_by: Option<Uuid>,
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
            created_by: row.get("created_by"),
        }
    }
}
