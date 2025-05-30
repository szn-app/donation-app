use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct ReviewQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ReviewQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_reviews(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::interaction::Review>> {
        log::debug!("--> list_reviews @ graphql resolver");
        let repository = repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let reviews = repository.list().await.map_err(|e| e.to_string())?;
        Ok(reviews)
    }
} 