use deadpool_postgres;
use std::error::Error;
use std::net::SocketAddr;
use std::net::ToSocketAddrs;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres;

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
        let mut rw_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(rw.ip().to_string());
            c.port(rw.port());
            c.dbname("app");
            c.user("postgres-user");
            c.password("postgres");
            c
        };

        let mut ro_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(ro.ip().to_string());
            c.port(ro.port());
            c.dbname("app");
            c.user("postgres-user");
            c.password("postgres");
            c
        };

        let mut r_config = {
            let mut c = tokio_postgres::Config::new();
            c.host(r.ip().to_string());
            c.port(r.port());
            c.dbname("app");
            c.user("postgres-user");
            c.password("postgres");
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

        let row: Vec<Row> = c
            .query("SELECT * FROM test LIMIT 10", &[])
            .await
            .expect("Query failed");

        // dbg!(row);

        Ok(())
    }
}
