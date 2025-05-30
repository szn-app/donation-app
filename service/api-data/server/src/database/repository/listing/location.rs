use crate::database::model::listing::{Location, GeoPoint, PostGisPoint};
use crate::database::sql::listing::location::{
    CREATE_LOCATION, DELETE_LOCATION, LIST_LOCATIONS, FIND_LOCATION,
    UPDATE_LOCATION,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;

pub struct LocationRepository {
    pool: PostgresPool,
}

impl LocationRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Location>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all locations");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_LOCATIONS, &[]).await?;
        Ok(rows.into_iter().map(Location::from).collect())
    }

    pub async fn find(&self, id: i64) -> Result<Option<Location>, Box<dyn Error + Send + Sync>> {
        debug!("Getting location by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_LOCATION, &[&id]).await?;
        Ok(row.map(Location::from))
    }

    pub async fn create(
        &self,
        address_line1: String,
        address_line2: Option<String>,
        city: String,
        state: String,
        district: Option<String>,
        country: String,
        geom: Option<GeoPoint>,
        entrance_note: Option<String>,
    ) -> Result<Location, Box<dyn Error + Send + Sync>> {
        debug!("Adding location: {}", address_line1);
        let client = self.pool.rw.get().await?;
        
        // Convert GeoPoint to PostGisPoint and then to the inner PostGIS point
        let postgis_point = geom.map(PostGisPoint::from).map(PostGisPoint::into_inner);
        
        let row = client
            .query_one(
                CREATE_LOCATION,
                &[
                    &address_line1,
                    &address_line2,
                    &city,
                    &state,
                    &district,
                    &country,
                    &postgis_point,
                    &entrance_note,
                ],
            )
            .await?;
        Ok(Location::from(row))
    }

    pub async fn update(
        &self,
        id: i64,
        address_line1: Option<String>,
        address_line2: Option<String>,
        city: Option<String>,
        state: Option<String>,
        district: Option<String>,
        country: Option<String>,
        geom: Option<GeoPoint>,
        entrance_note: Option<String>,
    ) -> Result<Location, Box<dyn Error + Send + Sync>> {
        debug!("Updating location: {}", id);
        let client = self.pool.rw.get().await?;
        
        // Convert GeoPoint to PostGisPoint and then to the inner PostGIS point
        let postgis_point = geom.map(PostGisPoint::from).map(PostGisPoint::into_inner);
        
        let row = client
            .query_one(
                UPDATE_LOCATION,
                &[
                    &id,
                    &address_line1,
                    &address_line2,
                    &city,
                    &state,
                    &district,
                    &country,
                    &postgis_point,
                    &entrance_note,
                ],
            )
            .await?;
        Ok(Location::from(row))
    }

    pub async fn delete(&self, id: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!("Deleting location: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_LOCATION, &[&id]).await?;
        Ok(())
    }
}
