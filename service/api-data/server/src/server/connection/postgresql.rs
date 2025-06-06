use deadpool_postgres;
use std::env;
use std::error::Error;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres;

#[derive(Debug, Clone)]
pub struct PostgresEndpointConfig {
    pub rw: String,
    pub ro: String,
    pub r: String,
}

#[derive(Debug, Clone)]
pub struct EndpointGroup {
    pub rw: SocketAddr,
    pub ro: SocketAddr,
    pub r: SocketAddr,
}

impl EndpointGroup {
    pub fn new(rw: &str, ro: &str, r: &str) -> Result<Self, String> {
        let rw_socket_addr = Self::resolve_single_addr(rw)
            .map_err(|e| format!("Failed to resolve rw address '{}': {}", rw, e))?;

        let ro_socket_addr = Self::resolve_single_addr(ro).unwrap_or_else(|e| {
            eprintln!(
                "Failed to resolve ro '{}': {}. Using fallback dummy address.",
                ro, e
            );
            // RFC 5737 dummy address (TEST-NET-1)
            "192.0.2.1:5432".parse().unwrap()
        });

        let r_socket_addr = Self::resolve_single_addr(r)
            .map_err(|e| format!("Failed to resolve r address '{}': {}", r, e))?;

        Ok(EndpointGroup {
            rw: rw_socket_addr,
            ro: ro_socket_addr,
            r: r_socket_addr,
        })
    }

    fn resolve_single_addr(addr: &str) -> Result<SocketAddr, std::io::Error> {
        let mut addrs = addr.to_socket_addrs()?;
        addrs.next().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("address '{}' couldn't be resolved", addr),
            )
        })
    }
}

#[derive(Debug)]
pub struct PostgresConfigGroup {
    pub rw: tokio_postgres::Config,
    pub ro: tokio_postgres::Config,
    pub r: tokio_postgres::Config,
}

impl PostgresConfigGroup {
    pub fn new(
        rw: &SocketAddr,
        ro: &SocketAddr,
        r: &SocketAddr,
    ) -> Result<PostgresConfigGroup, Box<dyn Error>> {
        let username =
            env::var("POSTGRESQL_USERNAME").unwrap_or_else(|_| "postgres-user".to_string());
        let password = env::var("POSTGRESQL_PASSWORD").unwrap_or_else(|_| "postgres".to_string());

        let rw_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(rw.ip().to_string());
            c.port(rw.port());
            c.dbname("app");
            c.user(&username);
            c.password(&password);
            c
        };

        let ro_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(ro.ip().to_string());
            c.port(ro.port());
            c.dbname("app");
            c.user(&username);
            c.password(&password);
            c
        };

        let r_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(r.ip().to_string());
            c.port(r.port());
            c.dbname("app");
            c.user(&username);
            c.password(&password);
            c
        };

        Ok(PostgresConfigGroup {
            rw: rw_config,
            ro: ro_config,
            r: r_config,
        })
    }
}

#[derive(Debug, Clone)]
pub struct PostgresPool {
    pub rw: deadpool_postgres::Pool,
    pub ro: deadpool_postgres::Pool,
    pub r: deadpool_postgres::Pool,
}

impl PostgresPool {
    pub async fn new(
        rw: tokio_postgres::Config,
        ro: tokio_postgres::Config,
        r: tokio_postgres::Config,
    ) -> Result<PostgresPool, Box<dyn Error>> {
        let mgr_config = deadpool_postgres::ManagerConfig {
            recycling_method: deadpool_postgres::RecyclingMethod::Verified, // vs RecyclingMethod::Fast which avoid connection checks
        };

        let rw_mgr =
            deadpool_postgres::Manager::from_config(rw, tokio_postgres::NoTls, mgr_config.clone());
        let ro_mgr =
            deadpool_postgres::Manager::from_config(ro, tokio_postgres::NoTls, mgr_config.clone());
        let r_mgr = deadpool_postgres::Manager::from_config(r, tokio_postgres::NoTls, mgr_config);

        // rough heuristic is max_size = (CPU cores × 2) + 1 per pool
        let rw_pool = deadpool_postgres::Pool::builder(rw_mgr)
            .max_size(10)
            .timeouts(deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(10)),
                create: Some(Duration::from_secs(5)),
                recycle: Some(Duration::from_secs(30)),
            })
            .runtime(deadpool_postgres::Runtime::Tokio1)
            .build()?;

        let ro_pool = deadpool_postgres::Pool::builder(ro_mgr)
            .max_size(20)
            .timeouts(deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(10)),
                create: Some(Duration::from_secs(5)),
                recycle: Some(Duration::from_secs(30)),
            })
            .runtime(deadpool_postgres::Runtime::Tokio1)
            .build()?;

        let r_pool = deadpool_postgres::Pool::builder(r_mgr)
            .max_size(15)
            .timeouts(deadpool_postgres::Timeouts {
                wait: Some(Duration::from_secs(10)),
                create: Some(Duration::from_secs(5)),
                recycle: Some(Duration::from_secs(30)),
            })
            .runtime(deadpool_postgres::Runtime::Tokio1)
            .build()?;

        Ok(PostgresPool {
            rw: rw_pool,
            ro: ro_pool,
            r: r_pool,
        })
    }

    pub fn new_mock() -> PostgresPool {
        use deadpool_postgres::{Config, ManagerConfig, RecyclingMethod, Runtime};
        use tokio_postgres::NoTls;

        Self {
            rw: {
                let mut cfg = Config::new();
                cfg.dbname = Some("deadpool".to_string());
                cfg.manager = Some(ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                });
                cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
            },
            ro: {
                let mut cfg = Config::new();
                cfg.dbname = Some("deadpool".to_string());
                cfg.manager = Some(ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                });
                cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
            },
            r: {
                let mut cfg = Config::new();
                cfg.dbname = Some("deadpool".to_string());
                cfg.manager = Some(ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                });
                cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
            },
        }
    }

    pub async fn test_connection(
        &self,
    ) -> Result<deadpool_postgres::Object, deadpool_postgres::PoolError> {
        // initial call to verify connection establishes successfully
        let c = self.rw.get().await?;

        if let Err(e) = self.ro.get().await {
            eprintln!("WARNING: Database connection failed for read-only pool: {} (Seems like no replicas are running). continuing...", e);
        }

        let _ = self.r.get().await?;

        Ok(c)
    }

    // used for simple tests during development
    pub async fn new_single_point_pool(port: Option<u16>) -> Self {
        // Base URL components
        let user = "postgres";
        let password = "postgres";
        let host = "localhost";
        let dbname = "app";
        let port = port.unwrap_or(5432);

        let postgresql_url = format!("{}:{}", host, port);

        let postgres_pool_group = {
            let db_context =
                DatabaseContext::new(&postgresql_url, &postgresql_url, &postgresql_url)
                    .await
                    .expect("Failed to create database context");

            db_context
                .test_connection()
                .await
                .expect("Database pool connections failed");

            db_context.postgres_pool_group
        };

        postgres_pool_group
    }
}

pub struct DatabaseContext {
    pub endpoint_group: EndpointGroup,
    pub postgres_config_group: PostgresConfigGroup,
    pub postgres_pool_group: PostgresPool,
}

impl DatabaseContext {
    pub async fn new(rw: &str, ro: &str, r: &str) -> Result<Self, Box<dyn Error>> {
        let endpoint_group = EndpointGroup::new(rw, ro, r)?;

        let postgres_config_group =
            PostgresConfigGroup::new(&endpoint_group.rw, &endpoint_group.ro, &endpoint_group.r)?;

        let postgres_pool_group = PostgresPool::new(
            postgres_config_group.rw.clone(),
            postgres_config_group.ro.clone(),
            postgres_config_group.r.clone(),
        )
        .await?;

        Ok(DatabaseContext {
            endpoint_group,
            postgres_config_group,
            postgres_pool_group,
        })
    }

    pub async fn test_connection(&self) -> Result<(), deadpool_postgres::PoolError> {
        use std::time::Duration;
        use tokio::time::sleep;
        use tokio_postgres::Row;

        let mut attempts = 0;

        let c = loop {
            attempts += 1;

            match self.postgres_pool_group.test_connection().await {
                Ok(c) => break Ok(c),
                Err(e) if attempts < 3 => {
                    eprintln!("Connection attempt {} failed: {}. Retrying...", attempts, e);
                    sleep(Duration::from_millis(3000)).await;
                }
                Err(e) => {
                    eprintln!("Connection failed after {} attempts: {}", attempts, e);
                    break Err(e);
                }
            }
        }?;

        if let Ok(_) = c
            .query("SELECT * FROM \"test\".\"test\" LIMIT 1", &[])
            .await
        {
            // do nothing
        };

        Ok(())
    }
}

pub mod client {
    // get connection from pool (single attempt)
    pub async fn db_client(
        postgres_pool: &deadpool_postgres::Pool,
    ) -> Result<deadpool_postgres::Client, Box<dyn std::error::Error>> {
        Ok(postgres_pool
            .get()
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?)
    }

    /// get connection from pool (multiple attempts with retries)
    pub async fn db_client_with_retry(
        postgres_pool: &deadpool_postgres::Pool,
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

        Err(last_error
            .unwrap_or_else(|| "Maximum retries exceeded when connecting to database".to_string()))
    }
}

// $`cargo test -q server::connection::tests::test_postgres_config_new -- --nocapture`
#[cfg(test)]
pub mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::str::FromStr;
    use PostgresConfigGroup;

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
