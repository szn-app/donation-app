// tokio_postgres::Row.get<_, T> method casts the SQL Postgresql value to a Rust type according to the mapping relying on FromSql/ToSql traits of rust-postgres > postgres-types crate https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html#types

use async_graphql;
use time;
use tokio_postgres::Row; // tokio-postgres struct representing values and row columns metadata
use uuid;

pub mod user {
    use super::*;

    #[derive(Debug, Clone, PartialEq, async_graphql::SimpleObject)]
    pub struct Account {
        pub id: uuid::Uuid,                   // uuid, NOT NULL
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
}

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
