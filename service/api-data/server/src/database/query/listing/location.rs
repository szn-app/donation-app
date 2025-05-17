use crate::database::model::listing::Location;
use crate::database::sql::{ADD_LOCATION, GET_LOCATIONS, GET_LOCATION_BY_ID, UPDATE_LOCATION};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct LocationRepository {
    pool: Pool,
}

impl LocationRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_locations(&self) -> Result<Vec<Location>> {
        debug!("Getting all locations");
        let client = self.pool.get().await?;
        let rows = client.query(GET_LOCATIONS, &[]).await?;
        Ok(rows.into_iter().map(Location::from).collect())
    }

    pub async fn get_location_by_id(&self, id: i64) -> Result<Option<Location>> {
        debug!("Getting location by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_LOCATION_BY_ID, &[&id]).await?;
        Ok(row.map(Location::from))
    }

    pub async fn add_location(
        &self,
        title: &str,
        description: &str,
        address: &str,
        created_by: Uuid,
    ) -> Result<Location> {
        debug!("Adding location: {}", title);
        let client = self.pool.get().await?;
        let row = client
            .query_one(ADD_LOCATION, &[&title, &description, &address, &created_by])
            .await?;
        Ok(Location::from(row))
    }

    pub async fn update_location(
        &self,
        id: i64,
        title: &str,
        description: &str,
        address: &str,
        updated_by: Uuid,
    ) -> Result<Location> {
        debug!("Updating location: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_LOCATION,
                &[&id, &title, &description, &address, &updated_by],
            )
            .await?;
        Ok(Location::from(row))
    }
}
