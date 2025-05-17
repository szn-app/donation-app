use crate::database::model::listing::Publish;
use crate::database::sql::{
    ADD_PUBLISH, DELETE_PUBLISH, GET_PUBLISHES, GET_PUBLISHES_BY_COLLECTION, GET_PUBLISHES_BY_ITEM,
    GET_PUBLISH_BY_ITEM_AND_COLLECTION, UPDATE_PUBLISH,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct PublishRepository {
    pool: Pool,
}

impl PublishRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_publishes(&self) -> Result<Vec<Publish>> {
        debug!("Getting all publishes");
        let client = self.pool.get().await?;
        let rows = client.query(GET_PUBLISHES, &[]).await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn get_publish_by_item_and_collection(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<Option<Publish>> {
        debug!(
            "Getting publish by item: {} and collection: {}",
            id_item, id_collection
        );
        let client = self.pool.get().await?;
        let row = client
            .query_opt(
                GET_PUBLISH_BY_ITEM_AND_COLLECTION,
                &[&id_item, &id_collection],
            )
            .await?;
        Ok(row.map(Publish::from))
    }

    pub async fn get_publishes_by_collection(&self, id_collection: i64) -> Result<Vec<Publish>> {
        debug!("Getting publishes by collection: {}", id_collection);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_PUBLISHES_BY_COLLECTION, &[&id_collection])
            .await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn get_publishes_by_item(&self, id_item: i64) -> Result<Vec<Publish>> {
        debug!("Getting publishes by item: {}", id_item);
        let client = self.pool.get().await?;
        let rows = client.query(GET_PUBLISHES_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn add_publish(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<&str>,
        position: i32,
        added_by: Uuid,
    ) -> Result<Publish> {
        debug!(
            "Adding publish for item: {} to collection: {}",
            id_item, id_collection
        );
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &added_by],
            )
            .await?;
        Ok(Publish::from(row))
    }

    pub async fn update_publish(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<&str>,
        position: i32,
        updated_by: Uuid,
    ) -> Result<Publish> {
        debug!(
            "Updating publish for item: {} in collection: {}",
            id_item, id_collection
        );
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &updated_by],
            )
            .await?;
        Ok(Publish::from(row))
    }

    pub async fn delete_publish(&self, id_item: i64, id_collection: i64) -> Result<()> {
        debug!(
            "Deleting publish for item: {} from collection: {}",
            id_item, id_collection
        );
        let client = self.pool.get().await?;
        client
            .execute(DELETE_PUBLISH, &[&id_item, &id_collection])
            .await?;
        Ok(())
    }
}
