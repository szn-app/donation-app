use crate::database::model::interaction::Schedule;
use crate::database::sql::{CREATE_SCHEDULE, LIST_SCHEDULES, FIND_SCHEDULE};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use time::OffsetDateTime;
use tokio_postgres::Row;
use tracing::debug;

pub struct ScheduleRepository {
    pool: PostgresPool,
}

impl ScheduleRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Schedule>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all schedules");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_SCHEDULES, &[]).await?;
        Ok(rows.into_iter().map(Schedule::from).collect())
    }

    pub async fn find(&self, id: i64) -> Result<Option<Schedule>, Box<dyn Error + Send + Sync>> {
        debug!("Getting schedule by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_SCHEDULE, &[&id]).await?;
        Ok(row.map(Schedule::from))
    }

    pub async fn create(
        &self,
        scheduled_for: OffsetDateTime,
    ) -> Result<Schedule, Box<dyn Error + Send + Sync>> {
        debug!("Adding schedule for: {}", scheduled_for);
        let client = self.pool.rw.get().await?;
        let row = client.query_one(CREATE_SCHEDULE, &[&scheduled_for]).await?;
        Ok(Schedule::from(row))
    }
}
