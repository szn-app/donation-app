use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct MediaQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MediaQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_media(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Media>> {
        let repository = repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository.list().await.map_err(|e| e.to_string())?;
        Ok(media)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_media(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Media>> {
        let repository = repository::listing::MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(media)
    }
} 