use crate::database::model::user::Account;
use crate::database::sql::user::account::{
    ADD_ACCOUNT, DELETE_ACCOUNT, GET_ACCOUNTS, GET_ACCOUNT_BY_EMAIL, GET_ACCOUNT_BY_ID,
    UPDATE_ACCOUNT,
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

    pub async fn get_accounts(&self) -> Result<Vec<Account>, Box<dyn Error>> {
        debug!("Getting all accounts");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_ACCOUNTS, &[]).await?;
        Ok(rows.into_iter().map(Account::from).collect())
    }

    pub async fn get_account_by_id(&self, id: Uuid) -> Result<Option<Account>, Box<dyn Error>> {
        debug!("Getting account by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_ACCOUNT_BY_ID, &[&id]).await?;
        Ok(row.map(Account::from))
    }

    pub async fn get_account_by_email(
        &self,
        email: &str,
    ) -> Result<Option<Account>, Box<dyn Error>> {
        debug!("Getting account by email: {}", email);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_ACCOUNT_BY_EMAIL, &[&email]).await?;
        Ok(row.map(Account::from))
    }

    pub async fn add_account(
        &self,
        email: &str,
        password_hash: &str,
        is_active: bool,
    ) -> Result<Account, Box<dyn Error>> {
        debug!("Adding account for email: {}", email);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_ACCOUNT, &[&email, &password_hash, &is_active])
            .await?;
        Ok(Account::from(row))
    }

    pub async fn update_account(
        &self,
        id: Uuid,
        email: Option<String>,
        password_hash: Option<String>,
        is_active: Option<bool>,
    ) -> Result<Account, Box<dyn Error>> {
        debug!("Updating account: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_ACCOUNT, &[&id, &email, &password_hash, &is_active])
            .await?;
        Ok(Account::from(row))
    }

    pub async fn delete_account(&self, id: Uuid) -> Result<(), Box<dyn Error>> {
        debug!("Deleting account: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_ACCOUNT, &[&id]).await?;
        Ok(())
    }
}
