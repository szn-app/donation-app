use crate::database::model::user::{Committee, CommitteeRole};
use crate::database::sql::{
    ADD_COMMITTEE, DELETE_COMMITTEE, GET_COMMITTEES, GET_COMMITTEES_BY_COMMUNITY,
    GET_COMMITTEES_BY_PROFILE, GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY, UPDATE_COMMITTEE_ROLE,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;

pub struct CommitteeRepository {
    pool: Pool,
}

impl CommitteeRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_committees(&self) -> Result<Vec<Committee>> {
        debug!("Getting all committees");
        let client = self.pool.get().await?;
        let rows = client.query(GET_COMMITTEES, &[]).await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn get_committee_by_profile_and_community(
        &self,
        id_profile: i64,
        id_community: i64,
    ) -> Result<Option<Committee>> {
        debug!(
            "Getting committee by profile: {} and community: {}",
            id_profile, id_community
        );
        let client = self.pool.get().await?;
        let row = client
            .query_opt(
                GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
                &[&id_profile, &id_community],
            )
            .await?;
        Ok(row.map(Committee::from))
    }

    pub async fn get_committees_by_community(&self, id_community: i64) -> Result<Vec<Committee>> {
        debug!("Getting committees by community: {}", id_community);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_COMMITTEES_BY_COMMUNITY, &[&id_community])
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn get_committees_by_profile(&self, id_profile: i64) -> Result<Vec<Committee>> {
        debug!("Getting committees by profile: {}", id_profile);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_COMMITTEES_BY_PROFILE, &[&id_profile])
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn add_committee(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee> {
        debug!(
            "Adding committee member: {} to community: {}",
            id_profile, id_community
        );
        let client = self.pool.get().await?;
        let row = client
            .query_one(ADD_COMMITTEE, &[&id_profile, &id_community, &member_role])
            .await?;
        Ok(Committee::from(row))
    }

    pub async fn update_committee_role(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee> {
        debug!(
            "Updating committee member role: {} in community: {}",
            id_profile, id_community
        );
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_COMMITTEE_ROLE,
                &[&id_profile, &id_community, &member_role],
            )
            .await?;
        Ok(Committee::from(row))
    }

    pub async fn delete_committee(&self, id_profile: i64, id_community: i64) -> Result<()> {
        debug!(
            "Deleting committee member: {} from community: {}",
            id_profile, id_community
        );
        let client = self.pool.get().await?;
        client
            .execute(DELETE_COMMITTEE, &[&id_profile, &id_community])
            .await?;
        Ok(())
    }
}
