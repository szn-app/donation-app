use crate::database::model::user::Profile;
use crate::database::sql::user::profile::{
    ADD_PROFILE, GET_PROFILES, GET_PROFILE_BY_ACCOUNT, GET_PROFILE_BY_ID, UPDATE_PROFILE,
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

    pub async fn get_profiles(&self) -> Result<Vec<Profile>, Box<dyn Error>> {
        debug!("Getting all profiles");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_PROFILES, &[]).await?;
        Ok(rows.into_iter().map(Profile::from).collect())
    }

    pub async fn get_profile_by_id(&self, id: Uuid) -> Result<Option<Profile>, Box<dyn Error>> {
        debug!("Getting profile by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_PROFILE_BY_ID, &[&id]).await?;
        Ok(row.map(Profile::from))
    }

    pub async fn get_profile_by_account(
        &self,
        id_account: Uuid,
    ) -> Result<Option<Profile>, Box<dyn Error>> {
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
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, Box<dyn Error>> {
        debug!("Adding profile for account: {}", id_account);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_PROFILE, &[&id_account, &name, &bio, &avatar_url])
            .await?;
        Ok(Profile::from(row))
    }

    pub async fn update_profile(
        &self,
        id: Uuid,
        name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile, Box<dyn Error>> {
        debug!("Updating profile: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_PROFILE, &[&id, &name, &bio, &avatar_url])
            .await?;
        Ok(Profile::from(row))
    }
}
