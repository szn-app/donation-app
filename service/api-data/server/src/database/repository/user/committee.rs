use crate::database::model::user::{Committee, CommitteeRole};
use crate::database::sql::user::committee::{
    ADD_COMMITTEE, DELETE_COMMITTEE, GET_COMMITTEES, GET_COMMITTEES_BY_COMMUNITY,
    GET_COMMITTEES_BY_PROFILE, GET_COMMITTEE_BY_ID, GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
    UPDATE_COMMITTEE_ROLE,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CommitteeRepository {
    pool: PostgresPool,
}

impl CommitteeRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_committees(&self) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all committees");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_COMMITTEES, &[]).await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn get_committee_by_id(
        &self,
        id_profile: Uuid,
        id_community: i64,
    ) -> Result<Option<Committee>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Getting committee by id_profile: {} and id_community: {}",
            id_profile, id_community
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(GET_COMMITTEE_BY_ID, &[&id_profile, &id_community])
            .await?;
        Ok(row.map(Committee::from))
    }

    pub async fn get_committee_by_profile_and_community(
        &self,
        id_profile: Uuid,
        id_community: i64,
    ) -> Result<Option<Committee>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Getting committee by profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
                &[&id_profile, &id_community],
            )
            .await?;
        Ok(row.map(Committee::from))
    }

    pub async fn get_committees_by_community(
        &self,
        id_community: i64,
    ) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Getting committees by community: {}", id_community);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_COMMITTEES_BY_COMMUNITY, &[&id_community])
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn get_committees_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Committee>, Box<dyn Error + Send + Sync>> {
        debug!("Getting committees by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_COMMITTEES_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn add_committee(
        &self,
        id_profile: Uuid,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee, Box<dyn Error + Send + Sync>> {
        debug!(
            "Adding committee for profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_COMMITTEE, &[&id_profile, &id_community, &member_role])
            .await?;
        Ok(Committee::from(row))
    }

    pub async fn update_committee_role(
        &self,
        id_profile: Uuid,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee, Box<dyn Error + Send + Sync>> {
        debug!(
            "Updating committee role for profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_COMMITTEE_ROLE,
                &[&id_profile, &id_community, &member_role],
            )
            .await?;
        Ok(Committee::from(row))
    }

    pub async fn delete_committee(
        &self,
        id_profile: Uuid,
        id_community: i64,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!(
            "Deleting committee for profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.rw.get().await?;
        client
            .execute(DELETE_COMMITTEE, &[&id_profile, &id_community])
            .await?;
        Ok(())
    }
}
