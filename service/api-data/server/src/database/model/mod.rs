// tokio_postgres::Row.get<_, T> method casts the SQL Postgresql value to a Rust type according to the mapping relying on FromSql/ToSql traits of rust-postgres > postgres-types crate https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html#types
// model examples https://github.com/async-graphql/examples/blob/a08ec5f730cdfb146ae6a662376ce112a189affc/models/starwars/src/model.rs#L108

use async_graphql;
use async_trait;
use time;
use tokio_postgres::Row; // tokio-postgres struct representing values and row columns metadata
use uuid;

pub mod user {
    use super::*;

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Account {
        /// kratos id value
        pub id: uuid::Uuid, // uuid, NOT NULL
        pub created_at: time::OffsetDateTime, // timestamp with time zone, NOT NULL
    }

    impl From<Row> for Account {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, uuid::Uuid>("id"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Profile {
        pub id: i64,
        pub name: Option<String>,
        pub description: Option<String>,
        pub type_: Option<String>,
        pub owner: uuid::Uuid,
        pub created_at: time::OffsetDateTime,
        pub updated_at: Option<time::OffsetDateTime>,
        pub created_by: uuid::Uuid,
    }

    impl From<Row> for Profile {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                name: row.get::<_, Option<String>>("name"),
                description: row.get::<_, Option<String>>("description"),
                type_: row.get::<_, Option<String>>("type"),
                owner: row.get::<_, uuid::Uuid>("owner"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, Option<time::OffsetDateTime>>("updated_at"),
                created_by: row.get::<_, uuid::Uuid>("created_by"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Community {
        pub id: i64,
        pub title: Option<String>,
        pub description: Option<String>,
        pub type_: String,
        pub owner: uuid::Uuid,
        pub created_at: time::OffsetDateTime,
        pub updated_at: Option<time::OffsetDateTime>,
        pub created_by: uuid::Uuid,
    }

    impl From<Row> for Community {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                title: row.get::<_, Option<String>>("title"),
                description: row.get::<_, Option<String>>("description"),
                type_: row.get::<_, String>("type"),
                owner: row.get::<_, uuid::Uuid>("owner"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, Option<time::OffsetDateTime>>("updated_at"),
                created_by: row.get::<_, uuid::Uuid>("created_by"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Committee {
        pub id_profile: i64,
        pub id_community: i64,
        pub member_role: String,
        pub joined_at: time::OffsetDateTime,
    }

    impl From<Row> for Committee {
        fn from(row: Row) -> Self {
            Self {
                id_profile: row.get::<_, i64>("id_profile"),
                id_community: row.get::<_, i64>("id_community"),
                member_role: row.get::<_, String>("member_role"),
                joined_at: row.get::<_, time::OffsetDateTime>("joined_at"),
            }
        }
    }
}

pub mod listing {
    use super::*;

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Category {
        pub id: i64,
        pub title: String,
        pub description: Option<String>,
        pub category_parent: Option<i64>,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Category {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                title: row.get::<_, String>("title"),
                description: row.get::<_, Option<String>>("description"),
                category_parent: row.get::<_, Option<i64>>("category_parent"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Location {
        pub id: i64,
        pub address_line1: String,
        pub address_line2: Option<String>,
        pub city: String,
        pub state: String,
        pub district: Option<String>,
        pub country: String,
        pub geom: Option<String>, // PostGIS geometry type
        pub entrance_note: Option<String>,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Location {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                address_line1: row.get::<_, String>("address_line1"),
                address_line2: row.get::<_, Option<String>>("address_line2"),
                city: row.get::<_, String>("city"),
                state: row.get::<_, String>("state"),
                district: row.get::<_, Option<String>>("district"),
                country: row.get::<_, String>("country"),
                geom: row.get::<_, Option<String>>("geom"),
                entrance_note: row.get::<_, Option<String>>("entrance_note"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Item {
        pub id: i64,
        pub id_category: i64,
        pub id_location: i64,
        pub title: String,
        pub description: Option<String>,
        pub condition: String,
        pub quantity: i32,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Item {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_category: row.get::<_, i64>("id_category"),
                id_location: row.get::<_, i64>("id_location"),
                title: row.get::<_, String>("title"),
                description: row.get::<_, Option<String>>("description"),
                condition: row.get::<_, String>("condition"),
                quantity: row.get::<_, i32>("quantity"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Collection {
        pub id: i64,
        pub id_community: i64,
        pub title: String,
        pub visibility: String,
        pub type_: String,
        pub position: i32,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Collection {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_community: row.get::<_, i64>("id_community"),
                title: row.get::<_, String>("title"),
                visibility: row.get::<_, String>("visibility"),
                type_: row.get::<_, String>("type"),
                position: row.get::<_, i32>("position"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Media {
        pub id: i64,
        pub id_item: i64,
        pub caption: Option<String>,
        pub url: String,
        pub type_: String,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Media {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_item: row.get::<_, i64>("id_item"),
                caption: row.get::<_, Option<String>>("caption"),
                url: row.get::<_, String>("url"),
                type_: row.get::<_, String>("type"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Publish {
        pub id_item: i64,
        pub id_collection: i64,
        pub note: Option<String>,
        pub position: i32,
        pub added_by: uuid::Uuid,
        pub created_at: time::OffsetDateTime,
        pub updated_at: time::OffsetDateTime,
    }

    impl From<Row> for Publish {
        fn from(row: Row) -> Self {
            Self {
                id_item: row.get::<_, i64>("id_item"),
                id_collection: row.get::<_, i64>("id_collection"),
                note: row.get::<_, Option<String>>("note"),
                position: row.get::<_, i32>("position"),
                added_by: row.get::<_, uuid::Uuid>("added_by"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, time::OffsetDateTime>("updated_at"),
            }
        }
    }
}

pub mod interaction {
    use super::*;

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Schedule {
        pub id: i64,
        pub scheduled_for: time::OffsetDateTime,
    }

    impl From<Row> for Schedule {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                scheduled_for: row.get::<_, time::OffsetDateTime>("scheduled_for"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Pledge {
        pub id: i64,
        pub id_profile: i64,
        pub id_item: i64,
        pub intent_action: String,
        pub message: Option<String>,
        pub status: String,
        pub pledged_at: time::OffsetDateTime,
        pub updated_at: Option<time::OffsetDateTime>,
        pub updated_by: Option<uuid::Uuid>,
    }

    impl From<Row> for Pledge {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_profile: row.get::<_, i64>("id_profile"),
                id_item: row.get::<_, i64>("id_item"),
                intent_action: row.get::<_, String>("intent_action"),
                message: row.get::<_, Option<String>>("message"),
                status: row.get::<_, String>("status"),
                pledged_at: row.get::<_, time::OffsetDateTime>("pledged_at"),
                updated_at: row.get::<_, Option<time::OffsetDateTime>>("updated_at"),
                updated_by: row.get::<_, Option<uuid::Uuid>>("updated_by"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Transaction {
        pub id: i64,
        pub id_pledge: i64,
        pub status: String,
        pub id_schedule: Option<i64>,
        pub id_location: Option<i64>,
        pub created_at: time::OffsetDateTime,
        pub updated_at: Option<time::OffsetDateTime>,
    }

    impl From<Row> for Transaction {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_pledge: row.get::<_, i64>("id_pledge"),
                status: row.get::<_, String>("status"),
                id_schedule: row.get::<_, Option<i64>>("id_schedule"),
                id_location: row.get::<_, Option<i64>>("id_location"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
                updated_at: row.get::<_, Option<time::OffsetDateTime>>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Message {
        pub id: i64,
        pub id_sender: Option<i64>,
        pub id_transaction: i64,
        pub type_: Option<String>,
        pub content: String,
        pub sent_at: time::OffsetDateTime,
        pub updated_at: Option<time::OffsetDateTime>,
    }

    impl From<Row> for Message {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                id_sender: row.get::<_, Option<i64>>("id_sender"),
                id_transaction: row.get::<_, i64>("id_transaction"),
                type_: row.get::<_, Option<String>>("type"),
                content: row.get::<_, String>("content"),
                sent_at: row.get::<_, time::OffsetDateTime>("sent_at"),
                updated_at: row.get::<_, Option<time::OffsetDateTime>>("updated_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Review {
        pub id_transaction: i64,
        pub id_subject_profile: Option<i64>,
        pub reviewer: Option<i64>,
        pub comment: Option<String>,
        pub score: i16,
        pub created_at: time::OffsetDateTime,
    }

    impl From<Row> for Review {
        fn from(row: Row) -> Self {
            Self {
                id_transaction: row.get::<_, i64>("id_transaction"),
                id_subject_profile: row.get::<_, Option<i64>>("id_subject_profile"),
                reviewer: row.get::<_, Option<i64>>("reviewer"),
                comment: row.get::<_, Option<String>>("comment"),
                score: row.get::<_, i16>("score"),
                created_at: row.get::<_, time::OffsetDateTime>("created_at"),
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct ScheduleOpportunity {
        pub id: i64,
        pub window_start: Option<time::OffsetDateTime>,
        pub window_end: Option<time::OffsetDateTime>,
    }

    impl From<Row> for ScheduleOpportunity {
        fn from(row: Row) -> Self {
            Self {
                id: row.get::<_, i64>("id"),
                window_start: row.get::<_, Option<time::OffsetDateTime>>("window_start"),
                window_end: row.get::<_, Option<time::OffsetDateTime>>("window_end"),
            }
        }
    }
}

pub mod test {
    use std::string;

    use super::*;

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Test {
        /// example graphql comments (should appear in graphql IDE)
        pub i: i32,
        pub s: String,
        pub d: time::OffsetDateTime,
    }

    impl From<Row> for Test {
        fn from(row: Row) -> Self {
            Self {
                i: row.get::<_, i32>("i"),
                s: row.get::<_, String>("s"),
                d: row.get::<_, time::OffsetDateTime>("d"),
            }
        }
    }
}

// TODO: use in implementation
mod example_enums {
    // https://docs.rs/postgres-types/latest/postgres_types/

    use postgres_types::{FromSql, ToSql};

    // Note: map the exact name - case sensitive - of the Postgres ENUM
    #[derive(FromSql, ToSql, Debug)]
    #[postgres(name = "USER_STATUS")]
    enum UserStatus {
        INACTIVE,
        ACTIVE,
        DELETED,
    }

    struct User {
        id: u32,
        name: String,
        age: Option<u32>,
        status: UserStatus,
    }
}
