use crate::database::model;
use crate::database::sql;
use crate::server::connection::{self, postgresql::client};
use deadpool_postgres::{Client, Pool};
use log;
use std::error::Error;
use tokio_postgres;
use uuid::Uuid;

pub mod user {
    use super::*;

    pub struct AccountRepository;

    impl AccountRepository {
        pub async fn get_accounts(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Account>, Box<dyn Error>> {
            log::debug!("--> get_accounts");

            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_ACCOUNTS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Iterate over the rows and convert each Row into an Account
            let accounts: Vec<model::user::Account> = rows
                .into_iter()
                .map(|row| row.into()) // Use the From<Row> for Account implementation
                .collect();

            Ok(accounts)
        }

        pub async fn add_account(
            postgres_pool_group: &connection::PostgresPool,
            id: uuid::Uuid,
        ) -> Result<model::user::Account, Box<dyn Error>> {
            log::debug!("--> add_account");

            let client = client::db_client_with_retry(&postgres_pool_group.rw).await?;

            // Insert user into database
            let row: tokio_postgres::Row = client
                .query_one(sql::ADD_ACCOUNT, &[&id])
                .await
                .map_err(|e| {
                    log::error!("Database error while adding user: {}", e);
                    e
                })?;

            let account: model::user::Account = row.into();

            Ok(account)
        }
    }
}

pub mod test {
    use super::*;

    pub struct TestRepository;

    impl TestRepository {
        pub async fn get_tests(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::test::Test>, Box<dyn Error>> {
            log::debug!("--> get_tests");

            // get connection from pool (single attempt)
            let client = (&postgres_pool_group.r)
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::GET_TESTS, &[])
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Iterate over the rows and convert each Row into an Account
            let tests: Vec<model::test::Test> = rows
                .into_iter()
                .map(|row| row.into()) // Use the From<Row> for Account implementation
                .collect();

            Ok(tests)
        }
    }
}

mod util {
    use super::*;

    fn log_full_db_err(err: &tokio_postgres::error::Error, msg: &str) {
        let dberr = match err.as_db_error() {
            None => {
                log::error!("Error unwrapping tokio_postgres DbError: {:?}", &err);
                return;
            }
            Some(err) => err,
        };
        log::error!(
            "DB error: {} {}",
            dberr.message(),
            dberr
                .detail()
                .expect("cannot retrieve detail error from postgres")
        );
        log::error!("{}", msg);
    }
}
