use tokio;

pub mod connection;

pub async fn run_server(rw: &str, ro: &str, r: &str) -> Result<(), Box<dyn std::error::Error>> {
    let postgres_pool_group = {
        let db_context = connection::DatabaseContext::new(rw, ro, r)
            .await
            .expect("Failed to create database context");

        db_context
            .test_connection()
            .await
            .expect("Database pool connections failed");

        db_context.postgres_pool_group
    };

    tokio::select! {
        result = crate::http::start_http_server(postgres_pool_group.clone()) => result,
        result = crate::grpc::start_grpc_server(postgres_pool_group) => result
    }
}

mod example {
    use super::*;

    async fn example_postgres_database_single_connection() -> Result<(), Box<dyn std::error::Error>>
    {
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

        let rows = client.query(query, &[]).await?;
        let results: Vec<i32> = rows.into_iter().map(|row| row.get(0)).collect();

        dbg!(results);

        Ok(())
    }
}

// $`cargo test -q server::tests::test_postgres_config_new -- --nocapture`
#[cfg(test)]
pub mod tests {
    use super::*;
    use connection::PostgresConfigGroup;
    use std::net::SocketAddr;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_postgres_config_new() {
        use std::net::ToSocketAddrs;

        let addrs = "api-data--cluster-db-rw.api-data.svc:5432"
            .parse()
            .or_else(|e| {
                eprintln!(
                    "Failed to parse & resolve ro '{}': {}. Using fallback dummy value",
                    "hostname", e
                );
                SocketAddr::from_str("192.0.2.1:5432")
            });

        return;

        // if let Err(e) = "api-data-cluster-db-rw.api-data.svc:5432".parse::<SocketAddr>() {
        //     eprintln!("{:?} {}", &e, e);
        //     panic!("");
        // };

        let rw: SocketAddr = "api-data--cluster-db-rw.api-data.svc:5432".parse().unwrap();
        let ro: SocketAddr = "api-data--cluster-db-ro.api-data.svc:5432".parse().unwrap();
        let r: SocketAddr = "localhost:5432".parse().unwrap();

        let result = PostgresConfigGroup::new(&rw, &ro, &r);
        assert!(result.is_ok(), "Failed to create PostgresConfigGroup");
    }
}
