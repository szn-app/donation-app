use crate::database::model::listing::Location;
use crate::database::sql::listing::location::{
    CREATE_LOCATION, DELETE_LOCATION, LIST_LOCATIONS, FIND_LOCATIONS_BY_PROFILE, FIND_LOCATION,
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

    pub async fn find_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Location>, Box<dyn Error + Send + Sync>> {
        debug!("Getting locations by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(FIND_LOCATIONS_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Location::from).collect())
    }

    pub async fn create(
        &self,
        name: &str,
        address: &str,
        city: &str,
        state: &str,
        country: &str,
        postal_code: &str,
        id_profile: Uuid,
    ) -> Result<Location, Box<dyn Error + Send + Sync>> {
        debug!("Adding location: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_LOCATION,
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

    pub async fn update(
        &self,
        id: i64,
        name: Option<String>,
        address: Option<String>,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
        postal_code: Option<String>,
    ) -> Result<Location, Box<dyn Error + Send + Sync>> {
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

    pub async fn delete(&self, id: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!("Deleting location: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_LOCATION, &[&id]).await?;
        Ok(())
    }
}
