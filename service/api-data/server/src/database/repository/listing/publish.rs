use crate::database::model::listing::Publish;
use crate::database::sql::listing::publish::{
    ADD_PUBLISH, DELETE_PUBLISH, GET_PUBLISHES, GET_PUBLISHES_BY_COLLECTION, GET_PUBLISHES_BY_ITEM,
    GET_PUBLISH_BY_ID, GET_PUBLISH_BY_ITEM_AND_COLLECTION, UPDATE_PUBLISH,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct PublishRepository {
    pool: PostgresPool,
}

impl PublishRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_publishes(&self) -> Result<Vec<Publish>, Box<dyn Error>> {
        debug!("Getting all publishes");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PUBLISHES, &[]).await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn get_publish_by_id(&self, id: i64) -> Result<Option<Publish>, Box<dyn Error>> {
        debug!("Getting publish by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_PUBLISH_BY_ID, &[&id]).await?;
        Ok(row.map(Publish::from))
    }

    pub async fn get_publish_by_item_and_collection(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<Option<Publish>, Box<dyn Error>> {
        debug!(
            "Getting publish by item: {} and collection: {}",
            id_item, id_collection
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_PUBLISH_BY_ITEM_AND_COLLECTION,
                &[&id_item, &id_collection],
            )
            .await?;
        Ok(row.map(Publish::from))
    }

    pub async fn get_publishes_by_collection(
        &self,
        id_collection: i64,
    ) -> Result<Vec<Publish>, Box<dyn Error>> {
        debug!("Getting publishes by collection: {}", id_collection);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_PUBLISHES_BY_COLLECTION, &[&id_collection])
            .await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn get_publishes_by_item(
        &self,
        id_item: i64,
    ) -> Result<Vec<Publish>, Box<dyn Error>> {
        debug!("Getting publishes by item: {}", id_item);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PUBLISHES_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn add_publish(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        created_by: Uuid,
    ) -> Result<Publish, Box<dyn Error>> {
        debug!(
            "Adding publish for item: {} in collection: {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &created_by],
            )
            .await?;
        Ok(Publish::from(row))
    }

    pub async fn update_publish(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        updated_by: Uuid,
    ) -> Result<Publish, Box<dyn Error>> {
        debug!(
            "Updating publish for item: {} in collection: {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_PUBLISH,
                &[&id_item, &id_collection, &note, &position, &updated_by],
            )
            .await?;
        Ok(Publish::from(row))
    }

    pub async fn delete_publish(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<(), Box<dyn Error>> {
        debug!(
            "Deleting publish for item: {} from collection: {}",
            id_item, id_collection
        );
        let client = self.pool.rw.get().await?;
        client
            .execute(DELETE_PUBLISH, &[&id_item, &id_collection])
            .await?;
        Ok(())
    }
}
