use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

pub struct AccountQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountQuery {
    /// Get all accounts
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_accounts(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let accounts = repository.list().await.map_err(|e| e.to_string())?;
        Ok(accounts)
    }

    /// Get account by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_account(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<Option<model::user::Account>> {
        log::debug!("--> account_by_id @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(account)
    }
} 