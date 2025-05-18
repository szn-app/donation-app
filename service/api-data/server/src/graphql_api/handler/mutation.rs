use crate::database::model;
use crate::database::query;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
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
        email: String,
        password_hash: String,
    ) -> FieldResult<model::user::Account> {
        log::debug!("--> add_account @ graphql resolver");

        let repository = query::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository
            .add_account(
                &email,
                &password_hash,
                true, // is_active
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(account)
    }
}
