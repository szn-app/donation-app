use crate::database::model::user::{Profile, ProfileType};
use crate::database::sql::user::profile::{
    ADD_PROFILE, DELETE_PROFILE, GET_PROFILES, GET_PROFILE_BY_ACCOUNT, GET_PROFILE_BY_ID,
    UPDATE_PROFILE,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct ProfileRepository {
    pool: PostgresPool,
}

impl ProfileRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_profiles(&self) -> Result<Vec<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all profiles");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PROFILES, &[]).await?;
        Ok(rows.into_iter().map(Profile::from).collect())
    }

    pub async fn get_profile_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Getting profile by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_PROFILE_BY_ID, &[&id]).await?;
        Ok(row.map(Profile::from))
    }

    pub async fn get_profile_by_account(
        &self,
        id_account: Uuid,
    ) -> Result<Option<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Getting profile by account: {}", id_account);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(GET_PROFILE_BY_ACCOUNT, &[&id_account])
            .await?;
        Ok(row.map(Profile::from))
    }

    pub async fn add_profile(
        &self,
        id_account: Uuid,
        name: &str,
        description: Option<String>,
        profile_type: Option<ProfileType>,
    ) -> Result<Profile, Box<dyn Error + Send + Sync>> {
        debug!("Adding profile for account: {}", id_account);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_PROFILE,
                &[&id_account, &name, &description, &profile_type],
            )
            .await?;
        Ok(Profile::from(row))
    }

    pub async fn update_profile(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        profile_type: Option<ProfileType>,
    ) -> Result<Profile, Box<dyn Error + Send + Sync>> {
        debug!("Updating profile: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_PROFILE, &[&id, &name, &description, &profile_type])
            .await?;
        Ok(Profile::from(row))
    }

    pub async fn delete_profile(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!("Deleting profile: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_PROFILE, &[&id]).await?;
        Ok(())
    }
}
