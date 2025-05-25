use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::community::{Community, CommunityType};
use crate::database::sql::user::community::{CREATE_COMMUNITY, GET_COMMUNITY_BY_ID, GET_COMMUNITIES_BY_OWNER, UPDATE_COMMUNITY, DELETE_COMMUNITY};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct CommunityRepository {
    pool: PostgresPool,
}

impl CommunityRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        title: String,
        description: Option<String>,
        type_: CommunityType,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Community, Error> {
        debug!("Creating new community: {} for owner: {}", title, owner);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                CREATE_COMMUNITY,
                &[&title, &description, &type_, &owner, &now, &created_by],
            )
            .await?;
        Ok(Community::from(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Community>, Error> {
        debug!("Getting community by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_COMMUNITY_BY_ID,
                &[&id],
            )
            .await?;
        Ok(row.map(Community::from))
    }

    pub async fn get_by_owner(&self, owner: Uuid) -> Result<Vec<Community>, Error> {
        debug!("Getting communities for owner: {}", owner);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                GET_COMMUNITIES_BY_OWNER,
                &[&owner],
            )
            .await?;
        Ok(rows.into_iter().map(Community::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        title: String,
        description: Option<String>,
        type_: CommunityType,
    ) -> Result<Option<Community>, Error> {
        debug!("Updating community {} with title: {}", id, title);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                UPDATE_COMMUNITY,
                &[&id, &title, &description, &type_, &now],
            )
            .await?;
        Ok(row.map(Community::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        debug!("Deleting community: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                DELETE_COMMUNITY,
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
