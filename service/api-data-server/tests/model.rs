use api_data::database::repository;
use api_data::database::sql::LIST_ACCOUNTS;
use std::error::Error;
use tokio_postgres::{self, NoTls};

#[cfg(feature = "run_dev_test")]
mod tests {
    use api_data::server::connection::{self, PostgresPool};
    use axum::routing::connect;
    use hyper::server::conn;

    use super::*;

    #[tokio::test]
    async fn example_postgres_database_single_connection() -> Result<(), Box<dyn std::error::Error>>
    {
        use tokio;
        use tokio_postgres;

        let url = "api-data--cluster-db-rw.api-data.svc:5432";
        let mut config = tokio_postgres::Config::new();
        let parts: Vec<&str> = url.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid URL format".into());
        }

        dbg!(&parts);

        config.host("api-data--cluster-db-rw.api-data.svc");
        config.port(5432);
        config.dbname("app");
        config.user("postgres-user");
        config.password("postgres");

        let query = "SELECT * FROM test LIMIT 10";
        let (client, connection) = config.connect(tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });

        let rows = client.query(repository, &[]).await?;
        let results: Vec<i32> = rows.into_iter().map(|row| row.get(0)).collect();

        dbg!(results);

        Ok(())
    }

    #[tokio::test]
    async fn test_db_connection_and_query() -> Result<(), tokio_postgres::Error> {
        let connection_string = "postgresql://postgres:postgres@localhost:5432/app";

        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!(
                    "Database must be exposed for the test. connection task error: {}",
                    e
                );
            }
        });

        let rows = client.query("SELECT 1", &[]).await?;

        assert!(!rows.is_empty(), "Expected at least one row from the query");

        let value: i32 = rows[0].get(0); // 1st column, 1st row

        assert_eq!(value, 1, "Expected query result to be 1");

        println!(
            "Database connection and query test passed! value: {:?}",
            rows[0].get::<usize, i32>(0)
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_query_1() -> Result<(), tokio_postgres::Error> {
        let connection_string = "postgresql://postgres:postgres@localhost:5432/app";

        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!(
                    "Database must be exposed for the test. connection task error: {}",
                    e
                );
            }
        });

        let rows = client.query(LIST_ACCOUNTS, &[]).await?;

        if !rows.is_empty() {
            let value: i32 = rows[0].get("id");

            assert_eq!(value, 1, "Expected query result to be 1");

            println!(
                "Database connection and query test passed! value: {:?}",
                rows[0].get::<usize, i32>(0)
            );
        } else {
            println!("Response has no rows");
        }

        Ok(())
    }

    //$ cargo test --features run_dev_test -q tests::test_query_2 -- --nocapture
    #[tokio::test]
    async fn test_query_2() -> Result<(), Box<dyn Error>> {
        let postgres_pool_group = PostgresPool::new_single_point_pool(Option::Some(5432)).await;

        let result =
            repository::user::AccountRepository::LIST_ACCOUNTS(&postgres_pool_group).await?;

        if !result.is_empty() {
            println!(
                "Database connection and query test passed!\n id: {:?}",
                result[0].id
            );
        } else {
            println!("Empty vector returned");
        }

        Ok(())
    }
}
