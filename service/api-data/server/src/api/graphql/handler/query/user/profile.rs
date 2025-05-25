use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct ProfileQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ProfileQuery {
    /// Get all profiles
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_profiles(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Profile>> {
        log::debug!("--> profiles @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profiles = repository.list().await.map_err(|e| e.to_string())?;
        Ok(profiles)
    }

    /// Get profile by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_profile(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::user::Profile>> {
        log::debug!("--> profile_by_id @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(profile)
    }
} 