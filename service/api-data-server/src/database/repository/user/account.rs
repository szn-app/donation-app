use log::debug;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::model::user::account::Account;
use crate::database::sql::user::account::{
    CREATE_ACCOUNT, DELETE_ACCOUNT, FIND_ACCOUNT, LIST_ACCOUNTS, UPDATE_ACCOUNT,
};
use crate::server::connection::PostgresPool;
use std::error::Error;

pub struct AccountRepository {
    pool: PostgresPool,
}

impl AccountRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    /// Retrieves all accounts from the database.
    pub async fn list(&self) -> Result<Vec<Account>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all accounts");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_ACCOUNTS, &[]).await?;
        Ok(rows.into_iter().map(Account::from).collect())
    }

    /// Retrieves an account by its ID.
    pub async fn find(&self, id: Uuid) -> Result<Option<Account>, Box<dyn Error + Send + Sync>> {
        debug!("Finding account by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_ACCOUNT, &[&id]).await?;
        Ok(row.map(Account::from))
    }

    /// Creates a new account with the specified ID.
    /// IMPORTANT: This method requires an explicit ID and should not be changed to auto-generate one. 
    /// The ID is a critical identifier that must be provided by the caller for proper integration.
    pub async fn create(
        &self,
        id: Uuid,
        remarks: Option<String>,
    ) -> Result<Account, Box<dyn Error + Send + Sync>> {
        debug!(
            "Creating new account with id: {} and remarks: {:?}",
            id, remarks
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(CREATE_ACCOUNT, &[&id, &remarks])
            .await?;
        Ok(Account::from(row))
    }

    /// Updates an existing account.
    pub async fn update(
        &self,
        id: Uuid,
        remarks: Option<String>,
    ) -> Result<Option<Account>, Box<dyn Error + Send + Sync>> {
        debug!("Updating account {} with remarks: {:?}", id, remarks);
        let client = self.pool.rw.get().await?;
        let row = client.query_opt(UPDATE_ACCOUNT, &[&id, &remarks]).await?;
        Ok(row.map(Account::from))
    }

    /// Deletes an account by its ID.
    pub async fn delete(&self, id: Uuid) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting account: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_ACCOUNT, &[&id]).await?;
        Ok(rows_affected > 0)
    }
}
