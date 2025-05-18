use crate::database::model::interaction::ScheduleOpportunity;
use crate::database::sql::{
    ADD_SCHEDULE_OPPORTUNITY, GET_SCHEDULE_OPPORTUNITIES, GET_SCHEDULE_OPPORTUNITY_BY_ID,
    UPDATE_SCHEDULE_OPPORTUNITY,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use time;
use tokio_postgres::Row;
use tracing::debug;

pub struct ScheduleOpportunityRepository {
    pool: PostgresPool,
}

impl ScheduleOpportunityRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_schedule_opportunities(
        &self,
    ) -> Result<Vec<ScheduleOpportunity>, Box<dyn Error>> {
        debug!("Getting all schedule opportunities");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_SCHEDULE_OPPORTUNITIES, &[]).await?;
        Ok(rows.into_iter().map(ScheduleOpportunity::from).collect())
    }

    pub async fn get_schedule_opportunity_by_id(
        &self,
        id: i64,
    ) -> Result<Option<ScheduleOpportunity>, Box<dyn Error>> {
        debug!("Getting schedule opportunity by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(GET_SCHEDULE_OPPORTUNITY_BY_ID, &[&id])
            .await?;
        Ok(row.map(ScheduleOpportunity::from))
    }

    pub async fn add_schedule_opportunity(
        &self,
        id_schedule: i64,
        id_opportunity: i64,
    ) -> Result<ScheduleOpportunity, Box<dyn Error>> {
        debug!(
            "Adding schedule opportunity for schedule: {} and opportunity: {}",
            id_schedule, id_opportunity
        );
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_SCHEDULE_OPPORTUNITY, &[&id_schedule, &id_opportunity])
            .await?;
        Ok(ScheduleOpportunity::from(row))
    }

    pub async fn update_schedule_opportunity(
        &self,
        id: i64,
        window_start: time::OffsetDateTime,
        window_end: time::OffsetDateTime,
    ) -> Result<ScheduleOpportunity, Box<dyn Error>> {
        debug!("Updating schedule opportunity: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_SCHEDULE_OPPORTUNITY,
                &[&id, &window_start, &window_end],
            )
            .await?;
        Ok(ScheduleOpportunity::from(row))
    }
}
