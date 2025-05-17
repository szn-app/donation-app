use crate::database::model::interaction::{Transaction, TransactionStatus};
use crate::database::sql::{
    ADD_TRANSACTION, GET_TRANSACTIONS, GET_TRANSACTIONS_BY_PLEDGE, GET_TRANSACTION_BY_ID,
    UPDATE_TRANSACTION,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;

pub struct TransactionRepository {
    pool: Pool,
}

impl TransactionRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_transactions(&self) -> Result<Vec<Transaction>> {
        debug!("Getting all transactions");
        let client = self.pool.get().await?;
        let rows = client.query(GET_TRANSACTIONS, &[]).await?;
        Ok(rows.into_iter().map(Transaction::from).collect())
    }

    pub async fn get_transaction_by_id(&self, id: i64) -> Result<Option<Transaction>> {
        debug!("Getting transaction by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_TRANSACTION_BY_ID, &[&id]).await?;
        Ok(row.map(Transaction::from))
    }

    pub async fn get_transactions_by_pledge(&self, id_pledge: i64) -> Result<Vec<Transaction>> {
        debug!("Getting transactions by pledge: {}", id_pledge);
        let client = self.pool.get().await?;
        let rows = client
            .query(GET_TRANSACTIONS_BY_PLEDGE, &[&id_pledge])
            .await?;
        Ok(rows.into_iter().map(Transaction::from).collect())
    }

    pub async fn add_transaction(
        &self,
        id_pledge: i64,
        status: TransactionStatus,
        id_schedule: Option<i64>,
        id_location: Option<i64>,
    ) -> Result<Transaction> {
        debug!("Adding transaction for pledge: {}", id_pledge);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                ADD_TRANSACTION,
                &[&id_pledge, &status, &id_schedule, &id_location],
            )
            .await?;
        Ok(Transaction::from(row))
    }

    pub async fn update_transaction(
        &self,
        id: i64,
        status: TransactionStatus,
        id_schedule: Option<i64>,
        id_location: Option<i64>,
    ) -> Result<Transaction> {
        debug!("Updating transaction: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_TRANSACTION,
                &[&id, &status, &id_schedule, &id_location],
            )
            .await?;
        Ok(Transaction::from(row))
    }
}
