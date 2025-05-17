use crate::database::model::interaction::ScheduleOpportunity;
use crate::database::sql::{
    ADD_SCHEDULE_OPPORTUNITY, GET_SCHEDULE_OPPORTUNITIES, GET_SCHEDULE_OPPORTUNITY_BY_ID,
    UPDATE_SCHEDULE_OPPORTUNITY,
};
use anyhow::Result;
use time::PrimitiveDateTime;
use tokio_postgres::Row;
use tracing::debug;

pub struct ScheduleOpportunityRepository {
    pool: deadpool_postgres::Pool,
}

impl ScheduleOpportunityRepository {
    pub fn new(pool: deadpool_postgres::Pool) -> Self {
        Self { pool }
    }

    pub async fn get_schedule_opportunities(&self) -> Result<Vec<ScheduleOpportunity>> {
        debug!("Getting all schedule opportunities");
        let client = self.pool.get().await?;
        let rows = client.query(GET_SCHEDULE_OPPORTUNITIES, &[]).await?;
        Ok(rows.into_iter().map(ScheduleOpportunity::from).collect())
    }

    pub async fn get_schedule_opportunity_by_id(
        &self,
        id: i64,
    ) -> Result<Option<ScheduleOpportunity>> {
        debug!("Getting schedule opportunity by id: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_opt(GET_SCHEDULE_OPPORTUNITY_BY_ID, &[&id])
            .await?;
        Ok(row.map(ScheduleOpportunity::from))
    }

    pub async fn add_schedule_opportunity(
        &self,
        window_start: PrimitiveDateTime,
        window_end: PrimitiveDateTime,
    ) -> Result<ScheduleOpportunity> {
        debug!("Adding schedule opportunity");
        let client = self.pool.get().await?;
        let row = client
            .query_one(ADD_SCHEDULE_OPPORTUNITY, &[&window_start, &window_end])
            .await?;
        Ok(ScheduleOpportunity::from(row))
    }

    pub async fn update_schedule_opportunity(
        &self,
        id: i64,
        window_start: PrimitiveDateTime,
        window_end: PrimitiveDateTime,
    ) -> Result<ScheduleOpportunity> {
        debug!("Updating schedule opportunity: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_SCHEDULE_OPPORTUNITY,
                &[&id, &window_start, &window_end],
            )
            .await?;
        Ok(ScheduleOpportunity::from(row))
    }
}
