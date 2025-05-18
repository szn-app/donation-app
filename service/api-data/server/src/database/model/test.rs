use async_graphql;
use deadpool_postgres::Client;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, async_graphql::SimpleObject /* defined for graphql schema */, Clone)]
pub struct Test {
    pub i: i32,
    pub s: String,
    pub d: OffsetDateTime,
}

// defined for the Postgresql to Rust type conversion
impl From<Row> for Test {
    fn from(row: Row) -> Self {
        Self {
            i: row.get("i"),
            s: row.get("s"),
            d: row.get("d"),
        }
    }
}
