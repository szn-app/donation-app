use crate::database::model::interaction::Schedule;
use crate::database::sql::{ADD_SCHEDULE, GET_SCHEDULES, GET_SCHEDULE_BY_ID, UPDATE_SCHEDULE};
use anyhow::Result;
use time::PrimitiveDateTime;
use tokio_postgres::Row;
use tracing::debug;

pub struct ScheduleRepository {
    pool: deadpool_postgres::Pool,
}

impl ScheduleRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn get_schedules(&self) -> Result<Vec<Schedule>> {
        debug!("Getting all schedules");
        let client = self.pool.get().await?;
        let rows = client.query(GET_SCHEDULES, &[]).await?;
        Ok(rows.into_iter().map(Schedule::from).collect())
    }

    pub async fn get_schedule_by_id(&self, id: i64) -> Result<Option<Schedule>> {
        debug!("Getting schedule by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_SCHEDULE_BY_ID, &[&id]).await?;
        Ok(row.map(Schedule::from))
    }

    pub async fn add_schedule(&self, scheduled_for: PrimitiveDateTime) -> Result<Schedule> {
        debug!("Adding schedule for: {}", scheduled_for);
        let client = self.pool.get().await?;
        let row = client.query_one(ADD_SCHEDULE, &[&scheduled_for]).await?;
        Ok(Schedule::from(row))
    }

    pub async fn update_schedule(
        &self,
        id: i64,
        scheduled_for: PrimitiveDateTime,
    ) -> Result<Schedule> {
        debug!("Updating schedule: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(UPDATE_SCHEDULE, &[&id, &scheduled_for])
            .await?;
        Ok(Schedule::from(row))
    }
}
