use crate::database::model::interaction::{Transaction, TransactionStatus};
use crate::database::sql::{
    CREATE_TRANSACTION, LIST_TRANSACTIONS, FIND_TRANSACTIONS_BY_PLEDGE, FIND_TRANSACTION,
    UPDATE_TRANSACTION,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct TransactionRepository {
    pool: PostgresPool,
}

impl TransactionRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all transactions");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_TRANSACTIONS, &[]).await?;
        Ok(rows.into_iter().map(Transaction::from).collect())
    }

    pub async fn find(
        &self,
        id: i64,
    ) -> Result<Option<Transaction>, Box<dyn Error + Send + Sync>> {
        debug!("Getting transaction by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_TRANSACTION, &[&id]).await?;
        Ok(row.map(Transaction::from))
    }

    pub async fn find_by_pledge(
        &self,
        id_pledge: i64,
    ) -> Result<Vec<Transaction>, Box<dyn Error + Send + Sync>> {
        debug!("Getting transactions by pledge: {}", id_pledge);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(FIND_TRANSACTIONS_BY_PLEDGE, &[&id_pledge])
            .await?;
        Ok(rows.into_iter().map(Transaction::from).collect())
    }

    pub async fn create(
        &self,
        id_pledge: i64,
        status: TransactionStatus,
        id_schedule: Option<i64>,
        id_location: Option<i64>,
    ) -> Result<Transaction, Box<dyn Error + Send + Sync>> {
        debug!("Adding transaction for pledge: {}", id_pledge);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_TRANSACTION,
                &[&id_pledge, &status, &id_schedule, &id_location],
            )
            .await?;
        Ok(Transaction::from(row))
    }

    pub async fn update(
        &self,
        id: i64,
        status: TransactionStatus,
    ) -> Result<Transaction, Box<dyn Error + Send + Sync>> {
        debug!("Updating transaction: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_TRANSACTION, &[&id, &status])
            .await?;
        Ok(Transaction::from(row))
    }
}
