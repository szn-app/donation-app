use crate::database::model::listing::Location;
use crate::database::sql::listing::location::{
    ADD_LOCATION, DELETE_LOCATION, GET_LOCATIONS, GET_LOCATIONS_BY_PROFILE, GET_LOCATION_BY_ID,
    UPDATE_LOCATION,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct LocationRepository {
    pool: PostgresPool,
}

impl LocationRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_locations(&self) -> Result<Vec<Location>, Box<dyn Error>> {
        debug!("Getting all locations");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_LOCATIONS, &[]).await?;
        Ok(rows.into_iter().map(Location::from).collect())
    }

    pub async fn get_location_by_id(&self, id: i64) -> Result<Option<Location>, Box<dyn Error>> {
        debug!("Getting location by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_LOCATION_BY_ID, &[&id]).await?;
        Ok(row.map(Location::from))
    }

    pub async fn get_locations_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Location>, Box<dyn Error>> {
        debug!("Getting locations by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_LOCATIONS_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Location::from).collect())
    }

    pub async fn add_location(
        &self,
        name: &str,
        address: &str,
        city: &str,
        state: &str,
        country: &str,
        postal_code: &str,
        id_profile: Uuid,
    ) -> Result<Location, Box<dyn Error>> {
        debug!("Adding location: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_LOCATION,
                &[
                    &name,
                    &address,
                    &city,
                    &state,
                    &country,
                    &postal_code,
                    &id_profile,
                ],
            )
            .await?;
        Ok(Location::from(row))
    }

    pub async fn update_location(
        &self,
        id: i64,
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
    ) -> Result<Location, Box<dyn Error>> {
        debug!("Updating location: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_LOCATION,
                &[&id, &name, &address, &city, &state, &country, &postal_code],
            )
            .await?;
        Ok(Location::from(row))
    }

    pub async fn delete_location(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting location: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_LOCATION, &[&id]).await?;
        Ok(())
    }
}
