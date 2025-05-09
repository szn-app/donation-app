use crate::database::model;
use crate::database::query;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use deadpool_postgres::Pool;
use log;
use uuid;

/// GraphQL Mutation Root
pub struct Mutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl Mutation {
    /// Create a new user
    async fn addAccount(
        &self,
        ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<model::user::Account> {
        log::debug!("--> add_account @ graphql resolver");

        let account = query::user::AccountRepository::add_account(&self.postgres_pool_group, id)
            .await
            .map_err(|e| e.to_string())?;

        Ok(account)
    }
}
