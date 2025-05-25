use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::user::Account;
use crate::database::repository::user::AccountRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use log;
use uuid::Uuid;

pub struct AccountMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id: Uuid,
        remarks: Option<String>,
    ) -> Result<Account> {
        log::debug!("--> create_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.create(id, remarks).await?;
        Ok(account)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn update(
        &self,
        _ctx: &Context<'_>,
        id: Uuid,
        remarks: Option<String>,
    ) -> Result<Option<Account>> {
        log::debug!("--> update_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.update(id, remarks).await?;
        Ok(account)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: Uuid) -> FieldResult<bool> {
        log::debug!("--> delete_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| e.to_string())?;
        Ok(result)
    }
} 