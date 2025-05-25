use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::profile::{Profile, ProfileType};
use crate::database::sql::user::profile::{CREATE_PROFILE, GET_PROFILE_BY_ID, GET_PROFILES_BY_OWNER, UPDATE_PROFILE, DELETE_PROFILE};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct ProfileRepository {
    pool: PostgresPool,
}

impl ProfileRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Profile, Error> {
        debug!("Creating new profile: {} for owner: {}", name, owner);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                CREATE_PROFILE,
                &[&name, &description, &type_, &owner, &now, &created_by],
            )
            .await?;
        Ok(Profile::from(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Profile>, Error> {
        debug!("Getting profile by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_PROFILE_BY_ID,
                &[&id],
            )
            .await?;
        Ok(row.map(Profile::from))
    }

    pub async fn get_by_owner(&self, owner: Uuid) -> Result<Vec<Profile>, Error> {
        debug!("Getting profiles for owner: {}", owner);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                GET_PROFILES_BY_OWNER,
                &[&owner],
            )
            .await?;
        Ok(rows.into_iter().map(Profile::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
    ) -> Result<Option<Profile>, Error> {
        debug!("Updating profile {} with name: {}", id, name);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                UPDATE_PROFILE,
                &[&id, &name, &description, &type_, &now],
            )
            .await?;
        Ok(row.map(Profile::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        debug!("Deleting profile: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                DELETE_PROFILE,
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
