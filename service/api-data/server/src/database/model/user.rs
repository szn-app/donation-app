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
#[postgres(name = "profile_type")]
pub enum ProfileType {
    Individual,
    Organization,
    Company,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "community_type")]
pub enum CommunityType {
    Solo,
    Organized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Enum, FromSql, ToSql)]
#[postgres(name = "committee_role")]
pub enum CommitteeRole {
    Organizer,
    Member,
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Account {
    pub id: Uuid,
    pub remarks: Option<String>,
    pub created_at: OffsetDateTime,
}

impl From<Row> for Account {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            remarks: row.get("remarks"),
            created_at: row.get("created_at"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Profile {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
    pub type_: Option<ProfileType>,
    pub owner: Uuid,
    pub created_at: OffsetDateTime,
    pub updated_at: Option<OffsetDateTime>,
    pub created_by: Uuid,
}

impl From<Row> for Profile {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            description: row.get("description"),
            type_: row.get("type"),
            owner: row.get("owner"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            created_by: row.get("created_by"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Community {
    pub id: i64,
    pub title: Option<String>,
    pub description: Option<String>,
    pub type_: CommunityType,
    pub owner: Uuid,
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

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Committee {
    pub id_profile: i64,
    pub id_community: i64,
    pub member_role: CommitteeRole,
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
