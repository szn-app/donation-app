use crate::database::model;
use crate::database::query;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use deadpool_postgres::Pool;
use log;

/// GraphQL Query Root
pub struct QueryResolver {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl QueryResolver {
    /// Get all accounts
    async fn accounts(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        // let c = ctx.data::<super::Context>()?; // EXMAPLE

        let account_list = query::user::AccountRepository::get_accounts(&self.postgres_pool_group)
            .await
            .map_err(|e| e.to_string())?;

        Ok(account_list)
    }
}
