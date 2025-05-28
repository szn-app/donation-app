use crate::api::graphql::guard::{auth, AuthorizeUser};
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
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    pub async fn create_community(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: Option<String>,
        variant: CommunityType,
        created_by: Uuid,
    ) -> Result<Community> {
        let community_repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = community_repository
            .create(name, description, variant, created_by, created_by)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(community)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn update_community(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        variant: Option<CommunityType>,
    ) -> FieldResult<Option<Community>> {
        log::debug!("--> update_community @ graphql resolver");
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .update(id, title, description, variant)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(community)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn delete_community(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        log::debug!("--> delete_community @ graphql resolver");
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let result = repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(result)
    }
}
