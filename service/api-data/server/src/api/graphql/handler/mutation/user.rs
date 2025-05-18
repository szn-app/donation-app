use crate::api::graphql::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

pub struct UserMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl UserMutation {
    /// Create a new account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_account(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<model::user::Account> {
        log::debug!("--> add_account @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository
            .add_account(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(account)
    }

    /// Delete an account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_account(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<bool> {
        log::debug!("--> delete_account @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_account(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
    }
}
