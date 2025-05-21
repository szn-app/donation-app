use crate::database::model::listing::{Collection, CollectionType};
use crate::database::sql::listing::collection::{
    ADD_COLLECTION, DELETE_COLLECTION, GET_COLLECTIONS, GET_COLLECTIONS_BY_PROFILE,
    GET_COLLECTION_BY_ID, UPDATE_COLLECTION,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CollectionRepository {
    pool: PostgresPool,
}

impl CollectionRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_collections(&self) -> Result<Vec<Collection>, Box<dyn Error>> {
        debug!("Getting all collections");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_COLLECTIONS, &[]).await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    pub async fn get_collection_by_id(
        &self,
        id: i64,
    ) -> Result<Option<Collection>, Box<dyn Error>> {
        debug!("Getting collection by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_COLLECTION_BY_ID, &[&id]).await?;
        Ok(row.map(Collection::from))
    }

    pub async fn get_collections_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Collection>, Box<dyn Error>> {
        debug!("Getting collections by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_COLLECTIONS_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    pub async fn add_collection(
        &self,
        name: &str,
        description: &str,
        id_profile: Uuid,
        is_public: bool,
        collection_type: CollectionType,
    ) -> Result<Collection, Box<dyn Error>> {
        debug!("Adding collection: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_COLLECTION,
                &[
                    &name,
                    &description,
                    &id_profile,
                    &is_public,
                    &collection_type,
                ],
            )
            .await?;
        Ok(Collection::from(row))
    }

    pub async fn update_collection(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        is_public: Option<bool>,
        collection_type: Option<CollectionType>,
    ) -> Result<Collection, Box<dyn Error>> {
        debug!("Updating collection: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_COLLECTION,
                &[&id, &name, &description, &is_public, &collection_type],
            )
            .await?;
        Ok(Collection::from(row))
    }

    pub async fn delete_collection(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting collection: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_COLLECTION, &[&id]).await?;
        Ok(())
    }
}
