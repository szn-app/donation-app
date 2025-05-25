use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::interaction::ScheduleOpportunity;
use crate::database::repository::interaction::schedule_opportunity::ScheduleOpportunityRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object};
use time::OffsetDateTime;
use tracing::debug;

pub struct ScheduleOpportunityMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleOpportunityMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn create(
        &self,
        _ctx: &Context<'_>,
        id_schedule: i64,
        id_opportunity: i64,
    ) -> FieldResult<ScheduleOpportunity> {
        debug!(
            "Creating schedule opportunity: schedule={}, opportunity={}",
            id_schedule, id_opportunity
        );
        let repository = ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let schedule_opportunity = repository
            .create(id_schedule, id_opportunity)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(schedule_opportunity)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        window_start: OffsetDateTime,
        window_end: OffsetDateTime,
    ) -> FieldResult<ScheduleOpportunity> {
        debug!("Updating schedule opportunity: id={}", id);
        let repository = ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let schedule_opportunity = repository
            .update(id, window_start, window_end)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(schedule_opportunity)
    }
} 