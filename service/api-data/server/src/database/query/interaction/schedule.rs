use crate::database::model::interaction::Schedule;
use crate::database::sql::{ADD_SCHEDULE, GET_SCHEDULES, GET_SCHEDULE_BY_ID};
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

    pub async fn get_schedules(&self) -> Result<Vec<Schedule>, Box<dyn Error>> {
        debug!("Getting all schedules");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_SCHEDULES, &[]).await?;
        Ok(rows.into_iter().map(Schedule::from).collect())
    }

    pub async fn get_schedule_by_id(&self, id: i64) -> Result<Option<Schedule>, Box<dyn Error>> {
        debug!("Getting schedule by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_SCHEDULE_BY_ID, &[&id]).await?;
        Ok(row.map(Schedule::from))
    }

    pub async fn add_schedule(
        &self,
        scheduled_for: OffsetDateTime,
    ) -> Result<Schedule, Box<dyn Error>> {
        debug!("Adding schedule for: {}", scheduled_for);
        let client = self.pool.rw.get().await?;
        let row = client.query_one(ADD_SCHEDULE, &[&scheduled_for]).await?;
        Ok(Schedule::from(row))
    }
}
