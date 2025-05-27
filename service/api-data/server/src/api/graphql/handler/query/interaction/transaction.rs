use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct TransactionQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl TransactionQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_transactions(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Transaction>> {
        log::debug!("--> list_transactions @ graphql resolver");
        let repository = repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transactions = repository.list().await.map_err(|e| e.to_string())?;
        Ok(transactions)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_transaction(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Transaction>> {
        log::debug!("--> find_transaction @ graphql resolver");
        let repository = repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(transaction)
    }
} 