use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::user::{Community, CommunityType};
use crate::database::repository::user::CommunityRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use log;
use uuid::Uuid;

pub struct CommunityMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommunityMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        avatar_url: Option<String>,
        created_by: Uuid,
    ) -> Result<Community> {
        let community_repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = community_repository
            .create(
                name,
                Some(description),
                CommunityType::Solo, // default type
                created_by,
                created_by,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(community)
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
        title: String,
        description: Option<String>,
        type_: CommunityType,
    ) -> FieldResult<Option<Community>> {
        log::debug!("--> update_community @ graphql resolver");
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .update(id, title, description, type_)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(community)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        log::debug!("--> delete_community @ graphql resolver");
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(result)
    }
} 