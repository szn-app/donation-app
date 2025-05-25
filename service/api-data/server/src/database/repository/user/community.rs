use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::community::{Community, CommunityType};
use crate::database::sql::user::community::{
    CREATE_COMMUNITY, DELETE_COMMUNITY, LIST_COMMUNITIES, FIND_COMMUNITIES_BY_OWNER, FIND_COMMUNITY,
    UPDATE_COMMUNITY,
};
use std::error::Error;
use crate::server::connection::PostgresPool;

pub struct CommunityRepository {
    pool: PostgresPool,
}

impl CommunityRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    /// Retrieves all communities from the database.
    pub async fn list(&self) -> Result<Vec<Community>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all communities");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_COMMUNITIES, &[]).await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    /// Retrieves a community by its ID.
    pub async fn find(&self, id: i64) -> Result<Option<Community>, Box<dyn Error + Send + Sync>> {
        debug!("Finding community by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_COMMUNITY, &[&id]).await?;
        Ok(row.map(Community::from))
    }

    /// Retrieves communities by owner.
    pub async fn find_by_owner(
        &self,
        owner: Uuid,
    ) -> Result<Vec<Community>, Box<dyn Error + Send + Sync>> {
        debug!("Finding communities by owner: {}", owner);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_COMMUNITIES_BY_OWNER, &[&owner]).await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    /// Creates a new community.
    pub async fn create(
        &self,
        title: String,
        description: Option<String>,
        type_: CommunityType,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Community, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating new community with title: {}, description: {:?}, type: {:?}, owner: {}, created_by: {}",
            title, description, type_, owner, created_by
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_COMMUNITY,
                &[&title, &description, &type_, &owner, &created_by],
            )
            .await?;
        Ok(Community::from(row))
    }

    /// Updates an existing community.
    pub async fn update(
        &self,
        id: i64,
        title: String,
        description: Option<String>,
        type_: CommunityType,
    ) -> Result<Option<Community>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Updating community {} with title: {}, description: {:?}, type: {:?}",
            id, title, description, type_
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(UPDATE_COMMUNITY, &[&id, &title, &description, &type_])
            .await?;
        Ok(row.map(Community::from))
    }

    /// Deletes a community by its ID.
    pub async fn delete(&self, id: i64) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting community: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_COMMUNITY, &[&id]).await?;
        Ok(rows_affected > 0)
    }
}
