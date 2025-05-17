use crate::database::model::user::{Community, CommunityType};
use crate::database::sql::{ADD_COMMUNITY, GET_COMMUNITIES, GET_COMMUNITY_BY_ID, UPDATE_COMMUNITY};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CommunityRepository {
    pool: Pool,
}

impl CommunityRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_communities(&self) -> Result<Vec<Community>> {
        debug!("Getting all communities");
        let client = self.pool.get().await?;
        let rows = client.query(GET_COMMUNITIES, &[]).await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    pub async fn get_community_by_id(&self, id: i64) -> Result<Option<Community>> {
        debug!("Getting community by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_COMMUNITY_BY_ID, &[&id]).await?;
        Ok(row.map(Community::from))
    }

    pub async fn add_community(
        &self,
        title: &str,
        description: &str,
        type_: CommunityType,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Community> {
        debug!("Adding community: {}", title);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_COMMUNITY,
                &[&title, &description, &type_, &owner, &created_by],
            )
            .await?;
        Ok(Community::from(row))
    }

    pub async fn update_community(
        &self,
        id: i64,
        title: &str,
        description: &str,
        type_: CommunityType,
        owner: Uuid,
        updated_by: Uuid,
    ) -> Result<Community> {
        debug!("Updating community: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_COMMUNITY,
                &[&id, &title, &description, &type_, &owner, &updated_by],
            )
            .await?;
        Ok(Community::from(row))
    }
}
