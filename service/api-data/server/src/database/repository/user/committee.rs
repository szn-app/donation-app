use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::committee::{Committee, CommitteeRole};
use crate::database::sql::user::committee::{
    CREATE_COMMITTEE, DELETE_COMMITTEE, FIND_COMMITTEE, FIND_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
    LIST_COMMITTEES, FIND_COMMITTEES_BY_COMMUNITY, FIND_COMMITTEES_BY_PROFILE, UPDATE_COMMITTEE_ROLE,
};
use std::error::Error;
use crate::server::connection::PostgresPool;

pub struct CommitteeRepository {
    pool: PostgresPool,
}

impl CommitteeRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    /// Retrieves all committees from the database.
    pub async fn list(&self) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all committees");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_COMMITTEES, &[]).await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    /// Retrieves a committee by its ID.
    pub async fn find(&self, id: Uuid) -> Result<Option<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Finding committee by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_COMMITTEE, &[&id]).await?;
        Ok(row.map(Committee::from))
    }

    /// Retrieves a committee by profile and community IDs.
    pub async fn find_by_profile_and_community(
        &self,
        id_profile: i64,
        id_community: i64,
    ) -> Result<Option<Committee>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Finding committee by profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(FIND_COMMITTEE_BY_PROFILE_AND_COMMUNITY, &[&id_profile, &id_community])
            .await?;
        Ok(row.map(Committee::from))
    }

    /// Retrieves committees by profile ID.
    pub async fn find_by_profile(
        &self,
        id_profile: i64,
    ) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Finding committees by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_COMMITTEES_BY_PROFILE, &[&id_profile]).await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    /// Retrieves committees by community ID.
    pub async fn find_by_community(
        &self,
        id_community: i64,
    ) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Finding committees by community: {}", id_community);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_COMMITTEES_BY_COMMUNITY, &[&id_community]).await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    /// Creates a new committee.
    pub async fn create(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating new committee with profile: {}, community: {}, role: {:?}",
            id_profile, id_community, member_role
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(CREATE_COMMITTEE, &[&id_profile, &id_community, &member_role])
            .await?;
        Ok(Committee::from(row))
    }

    /// Updates a committee's role.
    pub async fn update(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Option<Committee>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Updating committee role for profile: {}, community: {}, new role: {:?}",
            id_profile, id_community, member_role
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                UPDATE_COMMITTEE_ROLE,
                &[&id_profile, &id_community, &member_role],
            )
            .await?;
        Ok(row.map(Committee::from))
    }

    /// Deletes a committee by profile and community IDs.
    pub async fn delete(
        &self,
        id_profile: i64,
        id_community: i64,
    ) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!(
            "Deleting committee for profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(DELETE_COMMITTEE, &[&id_profile, &id_community])
            .await?;
        Ok(rows_affected > 0)
    }
}
