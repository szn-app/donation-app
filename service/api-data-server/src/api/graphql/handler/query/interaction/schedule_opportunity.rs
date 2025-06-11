use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct ScheduleOpportunityQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleOpportunityQuery {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn list_schedule_opportunities(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::ScheduleOpportunity>> {
        log::debug!("--> list_schedule_opportunities @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let opportunities = repository.list().await.map_err(|e| e.to_string())?;
        Ok(opportunities)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn find_schedule_opportunity(
        &self,
        ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::ScheduleOpportunity>> {
        log::debug!("--> find_schedule_opportunity @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let opportunity = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(opportunity)
    }
}
