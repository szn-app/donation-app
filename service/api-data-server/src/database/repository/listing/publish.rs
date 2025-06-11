use log::debug;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::model::listing::Publish;
use crate::database::sql::listing::publish::{
    CREATE_PUBLISH, DELETE_PUBLISH, FIND_PUBLISHES_BY_COLLECTION,
    FIND_PUBLISH_BY_ITEM_AND_COLLECTION, UPDATE_PUBLISH, LIST_PUBLISHES, FIND_PUBLISH
};
use crate::server::connection::PostgresPool;
use std::error::Error;


pub struct PublishRepository {
    pool: PostgresPool,
}
impl PublishRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        added_by: Uuid,
    ) -> Result<Publish, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating publish record for item {} in collection {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &added_by],
            )
            .await?;

        Ok(Publish::from(row))
    }

    pub async fn find_by_item_and_collection(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<Option<Publish>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Getting publish record for item {} in collection {}",
            id_item, id_collection
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                FIND_PUBLISH_BY_ITEM_AND_COLLECTION,
                &[&id_item, &id_collection],
            )
            .await?;
        Ok(row.map(Publish::from))
    }

    pub async fn find_by_collection(
        &self,
        id_collection: i64,
    ) -> Result<Vec<Publish>, Box<dyn Error + Send + Sync>> {
        debug!("Getting publish records for collection {}", id_collection);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(FIND_PUBLISHES_BY_COLLECTION, &[&id_collection])
            .await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn update(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
    ) -> Result<Publish, Box<dyn Error + Send + Sync>> {
        debug!(
            "Updating publish record for item {} in collection {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                UPDATE_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &now],
            )
            .await?;

        let publish = row.map(Publish::from).ok_or("No record found to update")?;

        Ok(publish)
    }

    pub async fn delete(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!(
            "Deleting publish record for item {} in collection {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(DELETE_PUBLISH, &[&id_item, &id_collection])
            .await?;
        Ok(rows_affected > 0)
    }

    pub async fn list(&self) -> Result<Vec<Publish>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all publish records");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_PUBLISHES, &[]).await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn find(&self, id: i64) -> Result<Publish, Box<dyn Error + Send + Sync>> {
        let client = self.pool.r.get().await?;
        let row = client.query_one(FIND_PUBLISH, &[&id]).await?;
        Ok(Publish::from(row))
    }
}
