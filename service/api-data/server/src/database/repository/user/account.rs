use crate::database::model::user::Account;
use crate::database::sql::user::account::{
    ADD_ACCOUNT, DELETE_ACCOUNT, GET_ACCOUNTS, GET_ACCOUNT_BY_ID,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct AccountRepository {
    pool: PostgresPool,
}

impl AccountRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_accounts(&self) -> Result<Vec<Account>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all accounts");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_ACCOUNTS, &[]).await?;
        Ok(rows.into_iter().map(Account::from).collect())
    }

    pub async fn get_account_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<Account>, Box<dyn Error + Send + Sync>> {
        debug!("Getting account by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_ACCOUNT_BY_ID, &[&id]).await?;
        Ok(row.map(Account::from))
    }

    pub async fn add_account(&self, id: Uuid) -> Result<Account, Box<dyn Error + Send + Sync>> {
        debug!("Adding account with id: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client.query_one(ADD_ACCOUNT, &[&id]).await?;
        Ok(Account::from(row))
    }

    pub async fn delete_account(&self, id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!("Deleting account: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_ACCOUNT, &[&id]).await?;
        Ok(())
    }
}
