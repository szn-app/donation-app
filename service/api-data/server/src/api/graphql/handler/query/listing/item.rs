use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct ItemQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ItemQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_items(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let items = repository.list().await.map_err(|e| e.to_string())?;
        Ok(items)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_item(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Item>> {
        let repository = repository::listing::ItemRepository::new(self.postgres_pool_group.clone());
        let item = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(item)
    }
} 