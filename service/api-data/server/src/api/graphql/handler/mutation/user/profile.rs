use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::user::{Profile, ProfileType};
use crate::database::repository::user::ProfileRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use log;
use uuid::Uuid;

pub struct ProfileMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ProfileMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id_account: Uuid,
        name: String,
        email: String,
        phone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Profile> {
        let profile_repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = profile_repository
            .create(
                name,
                Some(email),
                Some(ProfileType::Individual),
                id_account,
                id_account,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(profile)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
    ) -> FieldResult<Option<Profile>> {
        log::debug!("--> update_profile @ graphql resolver");
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .update(id, name, description, type_)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(profile)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        log::debug!("--> delete_profile @ graphql resolver");
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(result)
    }
} 