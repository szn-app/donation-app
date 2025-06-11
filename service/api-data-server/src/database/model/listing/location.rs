use async_graphql::{ComplexObject, Context, InputObject, SimpleObject};
use geo_types;
use postgis::{self}; // already implements FromSql, ToSql traits
use postgres_types::{FromSql, IsNull, ToSql, Type};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::error::Error;
use time::OffsetDateTime;
use tokio_postgres::Row;
use bytes;

/**
Convertion flow:
- CoordinatesInput -> GeoPoint -> PostGisPoint -> Database
- Database -> PostGisPoint -> GeoPoint -> Coordinates (for GraphQL)
*/

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
pub struct Coordinates {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, InputObject)]
pub struct CoordinatesInput {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Debug, Clone)]
pub struct GeoPoint(geo_types::Point<f64>);

#[derive(Debug, Clone, FromSql, ToSql)]
#[postgres(name = "geom")]
pub struct PostGisPoint(postgis::ewkb::Point);

impl PostGisPoint {
    pub fn into_inner(self) -> postgis::ewkb::Point {
        self.0
    }
}

// Implement serialization for PostGisPoint
impl Serialize for PostGisPoint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as a tuple of (x, y, srid)
        (&self.0.x, &self.0.y, &self.0.srid).serialize(serializer)
    }
}

// Implement deserialization for PostGisPoint
impl<'de> Deserialize<'de> for PostGisPoint {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // Deserialize from a tuple of (x, y, srid)
        let (x, y, srid) = <(f64, f64, Option<i32>)>::deserialize(deserializer)?;
        Ok(PostGisPoint(postgis::ewkb::Point { x, y, srid }))
    }
}

impl From<GeoPoint> for PostGisPoint {
    fn from(point: GeoPoint) -> Self {
        PostGisPoint(postgis::ewkb::Point {
            x: point.0.x(),
            y: point.0.y(),
            srid: Some(4326),
        })
    }
}

impl From<PostGisPoint> for GeoPoint {
    fn from(point: PostGisPoint) -> Self {
        GeoPoint(geo_types::Point::new(point.0.x, point.0.y))
    }
}

impl From<CoordinatesInput> for GeoPoint {
    fn from(coord: CoordinatesInput) -> Self {
        GeoPoint(geo_types::Point::new(coord.longitude, coord.latitude))
    }
}

impl From<GeoPoint> for Coordinates {
    fn from(point: GeoPoint) -> Self {
        Self {
            longitude: point.0.x(),
            latitude: point.0.y(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Location {
    pub id: i64,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub city: String,
    #[graphql(default)]
    pub state: Option<String>,
    pub district: Option<String>,
    pub country: String,
    #[graphql(skip)]
    pub geom: Option<PostGisPoint>,
    pub entrance_note: Option<String>,
    pub created_at: OffsetDateTime,
}

#[ComplexObject]
impl Location {
    async fn coordinates(&self) -> Option<Coordinates> {
        self.geom.clone().map(GeoPoint::from).map(Coordinates::from)
    }
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
            geom: row
                .get::<_, Option<postgis::ewkb::Point>>("geom")
                .map(|p| PostGisPoint(p)),
            entrance_note: row.get("entrance_note"),
            created_at: row.get("created_at"),
        }
    }
}
