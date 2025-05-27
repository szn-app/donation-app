use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::profile::{Profile, ProfileType};
use crate::database::sql::user::profile::{
    CREATE_PROFILE, DELETE_PROFILE, FIND_PROFILE, LIST_PROFILES, FIND_PROFILES_BY_OWNER,
    UPDATE_PROFILE,
};
use std::error::Error;
use crate::server::connection::PostgresPool;

pub struct ProfileRepository {
    pool: PostgresPool,
}

impl ProfileRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    /// Retrieves all profiles from the database.
    pub async fn list(&self) -> Result<Vec<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all profiles");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_PROFILES, &[]).await?;
        Ok(rows.into_iter().map(Profile::from).collect())
    }

    /// Retrieves a profile by its ID.
    pub async fn find(&self, id: i64) -> Result<Option<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Finding profile by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_PROFILE, &[&id]).await?;
        Ok(row.map(Profile::from))
    }

    /// Retrieves profiles by owner.
    pub async fn find_by_owner(
        &self,
        owner: Uuid,
    ) -> Result<Vec<Profile>, Box<dyn Error + Send + Sync>> {
        debug!("Finding profiles by owner: {}", owner);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_PROFILES_BY_OWNER, &[&owner]).await?;
        Ok(rows.into_iter().map(Profile::from).collect())
    }

    /// Creates a new profile.
    pub async fn create(
        &self,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
        owner: Uuid,
        created_by: Uuid,
    ) -> Result<Profile, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating new profile with name: {}, description: {:?}, type: {:?}, owner: {}, created_by: {}",
            name, description, type_, owner, created_by
        );
        let client = self.pool.rw.get().await?;

        // Convert Option<ProfileType> to a form tokio-postgres can handle Option<&ProfileType> with FromSql/ToSql blanket implementation
        let type_param: Option<&ProfileType> = type_.as_ref();

        let row = client
            .query_one(
                CREATE_PROFILE,
                &[&name, &description, &type_param, &owner, &created_by],
            )
            .await?;
        Ok(Profile::from(row))
    }

    /// Updates an existing profile.
    pub async fn update(
        &self,
        id: i64,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
    ) -> Result<Option<Profile>, Box<dyn Error + Send + Sync>> {
        debug!(
            "Updating profile {} with name: {}, description: {:?}, type: {:?}",
            id, name, description, type_
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(UPDATE_PROFILE, &[&id, &name, &description, &type_])
            .await?;
        Ok(row.map(Profile::from))
    }

    /// Deletes a profile by its ID.
    pub async fn delete(&self, id: i64) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting profile: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_PROFILE, &[&id]).await?;
        Ok(rows_affected > 0)
    }
}
