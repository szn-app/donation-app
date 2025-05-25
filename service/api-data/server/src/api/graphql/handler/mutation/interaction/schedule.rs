use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::interaction::Schedule;
use crate::database::repository::interaction::schedule::ScheduleRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object};
use time::OffsetDateTime;
use tracing::debug;

pub struct ScheduleMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn create(
        &self,
        _ctx: &Context<'_>,
        scheduled_for: OffsetDateTime,
    ) -> FieldResult<Schedule> {
        debug!("Creating schedule: scheduled_for={}", scheduled_for);
        let repository = ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository
            .create(scheduled_for)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(schedule)
    }
} 