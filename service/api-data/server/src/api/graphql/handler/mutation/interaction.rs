use crate::access_control::check_permission_for_subject;
use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use log;
use time;
use uuid;

pub struct ReviewMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ReviewMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_review(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_subject: i64,
        rating: i32,
        comment: Option<String>,
    ) -> FieldResult<model::interaction::Review> {
        log::debug!("--> add_review @ graphql resolver");
        let repository =
            repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let review = repository
            .add_review(id_transaction, id_subject, rating, comment)
            .await
            .map_err(|e| e.to_string())?;
        Ok(review)
    }
}

pub struct PledgeMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PledgeMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_pledge(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_item: i64,
        intent_action: model::listing::ItemIntentAction,
        message: Option<String>,
        status: model::interaction::PledgeStatus,
    ) -> FieldResult<model::interaction::Pledge> {
        log::debug!("--> add_pledge @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .add_pledge(id_profile, id_item, intent_action, message, status)
            .await
            .map_err(|e| e.to_string())?;
        Ok(pledge)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_pledge(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        status: model::interaction::PledgeStatus,
    ) -> FieldResult<model::interaction::Pledge> {
        log::debug!("--> update_pledge @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .update_pledge(id, status)
            .await
            .map_err(|e| e.to_string())?;
        Ok(pledge)
    }
}

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
    async fn add_schedule_opportunity(
        &self,
        _ctx: &Context<'_>,
        id_schedule: i64,
        id_opportunity: i64,
    ) -> FieldResult<model::interaction::ScheduleOpportunity> {
        log::debug!("--> add_schedule_opportunity @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let schedule_opportunity = repository
            .add_schedule_opportunity(id_schedule, id_opportunity)
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule_opportunity)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_schedule_opportunity(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        window_start: time::PrimitiveDateTime,
        window_end: time::PrimitiveDateTime,
    ) -> FieldResult<model::interaction::ScheduleOpportunity> {
        log::debug!("--> update_schedule_opportunity @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let schedule_opportunity = repository
            .update_schedule_opportunity(id, window_start, window_end)
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule_opportunity)
    }
}

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
    async fn add_schedule(
        &self,
        _ctx: &Context<'_>,
        scheduled_for: time::OffsetDateTime,
    ) -> FieldResult<model::interaction::Schedule> {
        log::debug!("--> add_schedule @ graphql resolver");
        let repository =
            repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository
            .add_schedule(scheduled_for)
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule)
    }
}

pub struct TransactionMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl TransactionMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_transaction(
        &self,
        _ctx: &Context<'_>,
        id_pledge: i64,
        status: model::interaction::TransactionStatus,
    ) -> FieldResult<model::interaction::Transaction> {
        log::debug!("--> add_transaction @ graphql resolver");
        let repository =
            repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository
            .add_transaction(id_pledge, status)
            .await
            .map_err(|e| e.to_string())?;
        Ok(transaction)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_transaction(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        status: model::interaction::TransactionStatus,
    ) -> FieldResult<model::interaction::Transaction> {
        log::debug!("--> update_transaction @ graphql resolver");
        let repository =
            repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository
            .update_transaction(id, status)
            .await
            .map_err(|e| e.to_string())?;
        Ok(transaction)
    }
}

pub struct MessageMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MessageMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_message(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_sender: uuid::Uuid,
        type_: model::interaction::MessageType,
        content: String,
    ) -> FieldResult<model::interaction::Message> {
        log::debug!("--> add_message @ graphql resolver");
        let repository =
            repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let message = repository
            .add_message(id_transaction, id_sender, type_, content)
            .await
            .map_err(|e| e.to_string())?;
        Ok(message)
    }
}
