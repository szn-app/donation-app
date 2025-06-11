use crate::database::model::test::Test;
use crate::database::sql;
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use log;
use std::error::Error;
use tokio_postgres::Row;

pub struct TestRepository {
    pool: PostgresPool,
}

impl TestRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Test>, Box<dyn Error + Send + Sync>> {
        log::debug!("--> get_tests");

        let client = self.pool.r.get().await?;
        let rows: Vec<Row> = client.query(sql::LIST_TESTS, &[]).await?;

        let tests: Vec<Test> = rows.into_iter().map(|row| row.into()).collect();

        Ok(tests)
    }
}
