use crate::database::model;
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

    pub async fn get_tests(&self) -> Result<Vec<model::test::Test>, Box<dyn Error>> {
        log::debug!("--> get_tests");

        let client = self.pool.r.get().await?;
        let rows: Vec<Row> = client.query(sql::GET_TESTS, &[]).await?;

        let tests: Vec<model::test::Test> = rows.into_iter().map(|row| row.into()).collect();

        Ok(tests)
    }
}
