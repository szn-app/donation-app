use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct CommunityQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommunityQuery {
    /// Get all communities
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_communities(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Community>> {
        log::debug!("--> communities @ graphql resolver");
        let repository = repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let communities = repository.list().await.map_err(|e| e.to_string())?;
        Ok(communities)
    }

    /// Get community by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_community(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::user::Community>> {
        log::debug!("--> community_by_id @ graphql resolver");
        let repository = repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(community)
    }
} 