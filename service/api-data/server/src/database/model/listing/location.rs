use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tokio_postgres::Row;
use geo_types::Point;

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Location {
    pub id: i64,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    pub state: String,
    pub district: Option<String>,
    pub country: String,
    #[graphql(skip)]
    pub geom: Option<Point<f64>>,
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