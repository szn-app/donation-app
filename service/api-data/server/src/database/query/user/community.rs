use crate::database::model::user::Community;
use crate::database::sql::user::community::{
    ADD_COMMUNITY, DELETE_COMMUNITY, GET_COMMUNITIES, GET_COMMUNITIES_BY_PROFILE,
    GET_COMMUNITY_BY_ID, UPDATE_COMMUNITY,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CommunityRepository {
    pool: PostgresPool,
}

impl CommunityRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_communities(&self) -> Result<Vec<Community>, Box<dyn Error>> {
        debug!("Getting all communities");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_COMMUNITIES, &[]).await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    pub async fn get_community_by_id(&self, id: i64) -> Result<Option<Community>, Box<dyn Error>> {
        debug!("Getting community by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_COMMUNITY_BY_ID, &[&id]).await?;
        Ok(row.map(Community::from))
    }

    pub async fn get_communities_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Community>, Box<dyn Error>> {
        debug!("Getting communities by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_COMMUNITIES_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    pub async fn add_community(
        &self,
        name: &str,
        description: &str,
        created_by: Uuid,
    ) -> Result<Community, Box<dyn Error>> {
        debug!("Adding community: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_COMMUNITY, &[&name, &description, &created_by])
            .await?;
        Ok(Community::from(row))
    }

    pub async fn update_community(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Community, Box<dyn Error>> {
        debug!("Updating community: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_COMMUNITY, &[&id, &name, &description])
            .await?;
        Ok(Community::from(row))
    }

    pub async fn delete_community(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting community: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_COMMUNITY, &[&id]).await?;
        Ok(())
    }
}
