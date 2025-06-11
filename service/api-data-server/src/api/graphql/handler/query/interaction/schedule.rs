use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct ScheduleQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_schedules(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Schedule>> {
        log::debug!("--> list_schedules @ graphql resolver");
        let repository = repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedules = repository.list().await.map_err(|e| e.to_string())?;
        Ok(schedules)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_schedule(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Schedule>> {
        log::debug!("--> find_schedule @ graphql resolver");
        let repository = repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(schedule)
    }
} 