// tokio_postgres::Row.get<_, T> method casts the SQL Postgresql value to a Rust type according to the mapping relying on FromSql/ToSql traits of rust-postgres > postgres-types crate https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html#types

use time;
use tokio_postgres::Row; // tokio-postgres struct representing values and row columns metadata
use uuid;

pub mod user {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
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
