use time::OffsetDateTime;
use uuid::Uuid;
use log::debug;

use crate::database::model::user::account::Account;
use crate::database::sql::user::account::{CREATE_ACCOUNT, GET_ACCOUNT_BY_ID, UPDATE_ACCOUNT, DELETE_ACCOUNT};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct AccountRepository {
    pool: PostgresPool,
}

impl AccountRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, remarks: Option<String>) -> Result<Account, Error> {
        debug!("Creating new account with remarks: {:?}", remarks);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_ACCOUNT,
                &[&Uuid::new_v4(), &remarks, &OffsetDateTime::now_utc()],
            )
            .await?;
        Ok(Account::from(row))
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<Account>, Error> {
        debug!("Getting account by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                GET_ACCOUNT_BY_ID,
                &[&id],
            )
            .await?;
        Ok(row.map(Account::from))
    }

    pub async fn update(&self, id: Uuid, remarks: Option<String>) -> Result<Option<Account>, Error> {
        debug!("Updating account {} with remarks: {:?}", id, remarks);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                UPDATE_ACCOUNT,
                &[&id, &remarks],
            )
            .await?;
        Ok(row.map(Account::from))
    }

    pub async fn delete(&self, id: Uuid) -> Result<bool, Error> {
        debug!("Deleting account: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                DELETE_ACCOUNT,
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
