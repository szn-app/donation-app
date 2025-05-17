use crate::database::model::listing::Collection;
use crate::database::sql::{
    ADD_COLLECTION, GET_COLLECTIONS, GET_COLLECTIONS_BY_COMMUNITY, GET_COLLECTION_BY_ID,
    UPDATE_COLLECTION,
};
use anyhow::Result;
use tokio_postgres::Row;
use tracing::debug;
use uuid;

pub struct CollectionRepository {
    pool: deadpool_postgres::Pool,
}

impl CollectionRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn get_collections(&self) -> Result<Vec<Collection>> {
        debug!("Getting all collections");
        let client = self.pool.get().await?;
        let rows = client.query(GET_COLLECTIONS, &[]).await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    pub async fn get_collection_by_id(&self, id: i64) -> Result<Option<Collection>> {
        debug!("Getting collection by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_COLLECTION_BY_ID, &[&id]).await?;
        Ok(row.map(Collection::from))
    }

    pub async fn get_collections_by_community(&self, id_community: i64) -> Result<Vec<Collection>> {
        debug!("Getting collections by community: {}", id_community);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_COLLECTIONS_BY_COMMUNITY, &[&id_community])
            .await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    pub async fn add_collection(
        &self,
        title: &str,
        description: &str,
        id_community: i64,
        created_by: uuid::Uuid,
    ) -> Result<Collection> {
        debug!("Adding collection: {}", title);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_COLLECTION,
                &[&title, &description, &id_community, &created_by],
            )
            .await?;
        Ok(Collection::from(row))
    }

    pub async fn update_collection(
        &self,
        id: i64,
        title: &str,
        description: &str,
        id_community: i64,
        updated_by: uuid::Uuid,
    ) -> Result<Collection> {
        debug!("Updating collection: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_COLLECTION,
                &[&id, &title, &description, &id_community, &updated_by],
            )
            .await?;
        Ok(Collection::from(row))
    }
}
