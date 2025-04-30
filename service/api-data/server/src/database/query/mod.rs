use crate::database::model;
use crate::database::sql;
use crate::server::connection;
use deadpool_postgres::{Client, Pool};
use log;
use std::error::Error;
use tokio_postgres;
use uuid::Uuid;

pub mod user {
    use super::*;

    pub struct AccountRepository;

    impl AccountRepository {
        // get connection from pool (single attempt)
        async fn db_client(postgres_pool: &Pool) -> Result<Client, Box<dyn std::error::Error>> {
            Ok(postgres_pool
                .get()
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?)
        }

        /// get connection from pool (multiple attempts with retries)
        async fn db_client_with_retry(
            postgres_pool: &Pool,
        ) -> Result<deadpool_postgres::Object, String> {
            use retry::{delay::Exponential, retry};
            use std::time::Duration;
            use tokio::time::sleep;

            let delay = Exponential::from_millis(100).take(3);
            let mut last_error = None;

            for delay_duration in delay {
                match postgres_pool.get().await {
                    Ok(client) => return Ok(client),
                    Err(e) => {
                        let error_msg = format!("Failed to get DB connection: {}", e);
                        log::debug!("{}", error_msg);
                        last_error = Some(error_msg);
                        sleep(delay_duration).await;
                    }
                }
            }

            Err(last_error.unwrap_or_else(|| {
                "Maximum retries exceeded when connecting to database".to_string()
            }))
        }

        pub async fn get_accounts(
            postgres_pool_group: &connection::PostgresPool,
        ) -> Result<Vec<model::user::Account>, Box<dyn Error>> {
            let client = Self::db_client(&postgres_pool_group.ro).await?;

            let rows: Vec<tokio_postgres::Row> = client
                .query(sql::SQL_GET_ACCOUNTS, &[])
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
            user_uuid: uuid::Uuid,
        ) -> Result<(), Box<dyn Error>> {
            let client = Self::db_client_with_retry(&postgres_pool_group.rw).await?;

            // Insert user into database
            client
                .execute(sql::SQL_ADD_ACCOUNT, &[&user_uuid])
                .await
                .map_err(|e| {
                    log::error!("Database error while adding user: {}", e);
                    e
                })?;

            Ok(())
        }
    }
}
