// tokio_postgres::Row.get<_, T> method casts the SQL Postgresql value to a Rust type according to the mapping relying on FromSql/ToSql traits of rust-postgres > postgres-types crate https://docs.rs/tokio-postgres/latest/tokio_postgres/types/trait.FromSql.html#types
// model examples https://github.com/async-graphql/examples/blob/a08ec5f730cdfb146ae6a662376ce112a189affc/models/starwars/src/model.rs#L108

use async_graphql;
use async_trait;
use time;
use tokio_postgres::Row; // tokio-postgres struct representing values and row columns metadata
use uuid;

pub mod interaction;
pub mod listing;
pub mod test;
pub mod user;

pub use interaction::*;
pub use listing::*;
pub use test::*;
pub use user::*;

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
