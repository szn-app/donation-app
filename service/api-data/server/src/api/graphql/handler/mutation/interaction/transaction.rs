use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::interaction::{Transaction, TransactionStatus};
use crate::database::repository::interaction::transaction::TransactionRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object};
use tracing::debug;

pub struct TransactionMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl TransactionMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn create(
        &self,
        _ctx: &Context<'_>,
        id_pledge: i64,
        status: TransactionStatus,
        id_schedule: Option<i64>,
        id_location: Option<i64>,
    ) -> FieldResult<Transaction> {
        debug!("Creating transaction: pledge={}", id_pledge);
        let transaction_repo = TransactionRepository::new(self.postgres_pool_group.clone());

        let transaction = transaction_repo
            .create(id_pledge, status, id_schedule, id_location)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(transaction)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        status: TransactionStatus,
    ) -> FieldResult<Transaction> {
        debug!("Updating transaction: id={}", id);
        let repository = TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository
            .update(id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(transaction)
    }
} 