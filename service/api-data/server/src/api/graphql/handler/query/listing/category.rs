use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct CategoryQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CategoryQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_categories(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::Category>> {
        let repository = repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let categories = repository.list().await.map_err(|e| e.to_string())?;
        Ok(categories)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_category(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Category>> {
        let repository = repository::listing::CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(category)
    }
} 