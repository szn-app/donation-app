use log::debug;
use time::OffsetDateTime;

use crate::database::model::listing::{Collection, CollectionType, CollectionVisibility};
use crate::database::sql::listing::collection::{
    CREATE_COLLECTION, DELETE_COLLECTION, FIND_COLLECTION, LIST_COLLECTIONS, UPDATE_COLLECTION,
};
use crate::server::connection::PostgresPool;
use std::error::Error;

pub struct CollectionRepository {
    pool: PostgresPool,
}

impl CollectionRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_community: i64,
        title: String,
        visibility: CollectionVisibility,
        variant: CollectionType,
        position: i32,
    ) -> Result<Collection, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating collection {} for community {}",
            title, id_community
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_COLLECTION,
                &[&id_community, &title, &visibility, &variant, &position],
            )
            .await?;
        Ok(Collection::from(row))
    }

    pub async fn find(&self, id: i64) -> Result<Option<Collection>, Box<dyn Error + Send + Sync>> {
        debug!("Getting collection by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_COLLECTION, &[&id]).await?;
        Ok(row.map(Collection::from))
    }

    pub async fn update(
        &self,
        id: i64,
        title: String,
        visibility: CollectionVisibility,
        variant: CollectionType,
        position: i32,
    ) -> Result<Collection, Box<dyn Error + Send + Sync>> {
        debug!("Updating collection {} with title: {}", id, title);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                UPDATE_COLLECTION,
                &[&id, &title, &visibility, &variant, &position, &now],
            )
            .await?;

        let collection = row
            .map(Collection::from)
            .ok_or("No record found to update")?;

        Ok(collection)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting collection: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_COLLECTION, &[&id]).await?;
        Ok(rows_affected > 0)
    }

    pub async fn list(&self) -> Result<Vec<Collection>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all collections");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_COLLECTIONS, &[]).await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }
}
