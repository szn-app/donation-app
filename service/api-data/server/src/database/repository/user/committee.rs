use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::committee::{Committee, CommitteeRole};
use crate::database::sql::user::committee::{
    CREATE_COMMITTEE, GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
    GET_COMMITTEES_BY_PROFILE, GET_COMMITTEES_BY_COMMUNITY,
    UPDATE_COMMITTEE_ROLE, DELETE_COMMITTEE
};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct CommitteeRepository {
    pool: PostgresPool,
}

impl CommitteeRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee, Error> {
        debug!("Creating committee for profile: {} in community: {} with role: {:?}", id_profile, id_community, member_role);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                CREATE_COMMITTEE,
                &[&id_profile, &id_community, &member_role, &now],
            )
            .await?;
        Ok(Committee::from(row))
    }

    pub async fn get_by_profile_and_community(
        &self,
        id_profile: i64,
        id_community: i64,
    ) -> Result<Option<Committee>, Error> {
        debug!("Getting committee for profile: {} in community: {}", id_profile, id_community);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_COMMITTEE_BY_PROFILE_AND_COMMUNITY,
                &[&id_profile, &id_community],
            )
            .await?;
        Ok(row.map(Committee::from))
    }

    pub async fn get_by_profile(&self, id_profile: i64) -> Result<Vec<Committee>, Error> {
        debug!("Getting committees for profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                GET_COMMITTEES_BY_PROFILE,
                &[&id_profile],
            )
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn get_by_community(&self, id_community: i64) -> Result<Vec<Committee>, Error> {
        debug!("Getting committees for community: {}", id_community);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                GET_COMMITTEES_BY_COMMUNITY,
                &[&id_community],
            )
            .await?;
        Ok(rows.into_iter().map(Committee::from).collect())
    }

    pub async fn update_role(
        &self,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Option<Committee>, Error> {
        debug!("Updating committee role for profile: {} in community: {} to: {:?}", id_profile, id_community, member_role);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                UPDATE_COMMITTEE_ROLE,
                &[&id_profile, &id_community, &member_role],
            )
            .await?;
        Ok(row.map(Committee::from))
    }

    pub async fn delete(
        &self,
        id_profile: i64,
        id_community: i64,
    ) -> Result<bool, Error> {
        debug!("Deleting committee for profile: {} in community: {}", id_profile, id_community);
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                DELETE_COMMITTEE,
                &[&id_profile, &id_community],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
