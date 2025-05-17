use crate::database::model::user::{Profile, ProfileType};
use crate::database::sql::{ADD_PROFILE, GET_PROFILES, GET_PROFILE_BY_ID, UPDATE_PROFILE};
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

    pub async fn get_profile_by_id(&self, id: i64) -> Result<Option<Profile>, Box<dyn Error>> {
        debug!("Getting profile by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_PROFILE_BY_ID, &[&id]).await?;
        Ok(row.map(Profile::from))
    }

    pub async fn add_profile(
        &self,
        name: &str,
        description: &str,
        type_: Option<ProfileType>,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Profile, Box<dyn Error>> {
        debug!("Adding profile: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_PROFILE,
                &[&name, &description, &type_, &owner, &created_by],
            )
            .await?;
        Ok(Profile::from(row))
    }

    pub async fn update_profile(
        &self,
        id: i64,
        name: &str,
        description: &str,
        type_: Option<ProfileType>,
    ) -> Result<Profile, Box<dyn Error>> {
        debug!("Updating profile: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_PROFILE, &[&id, &name, &description, &type_])
            .await?;
        Ok(Profile::from(row))
    }
}
