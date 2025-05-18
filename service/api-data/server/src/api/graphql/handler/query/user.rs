use crate::database::model;
use crate::database::repository;
use crate::api::graphql::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::server::connection::{KetoChannelGroup, PostgresPool};

use async_graphql::{self, Context, Error, ErrorExtensions, FieldResult, Object}; // note: `graphql` attribute is processed by async_graphql macros
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;
use time;
use uuid;

pub struct UserQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl UserQuery {
    /// Get all accounts
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn accounts(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let accounts = repository.get_accounts().await.map_err(|e| e.to_string())?;
        Ok(accounts)
    }

    /// Get account by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn account_by_id(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<Option<model::user::Account>> {
        log::debug!("--> account_by_id @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository
            .get_account_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(account)
    }
}
