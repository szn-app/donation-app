use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct PledgeQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PledgeQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_pledges(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::interaction::Pledge>> {
        log::debug!("--> list_pledges @ graphql resolver");
        let repository = repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledges = repository.list().await.map_err(|e| e.to_string())?;
        Ok(pledges)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_pledge(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::interaction::Pledge>> {
        log::debug!("--> find_pledge @ graphql resolver");
        let repository = repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(pledge)
    }
} 