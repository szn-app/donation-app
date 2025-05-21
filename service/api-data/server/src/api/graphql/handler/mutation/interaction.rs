use crate::access_control::check_permission_for_subject;
use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::interaction::{
    Message, MessageType, Pledge, PledgeIntentAction, PledgeStatus, Review, Schedule,
    ScheduleOpportunity, Transaction, TransactionStatus,
};
use crate::database::repository::interaction::{
    message::MessageRepository, pledge::PledgeRepository, review::ReviewRepository,
    schedule::ScheduleRepository, schedule_opportunity::ScheduleOpportunityRepository,
    transaction::TransactionRepository,
};
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, Error, FieldResult, Object, Result};
use log;
use time;
use tracing::debug;
use tracing::instrument;
use uuid::Uuid;

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
    pub async fn add_review(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_subject: i64,
        rating: i32,
        comment: Option<String>,
    ) -> Result<Review> {
        let review_repository = ReviewRepository::new(self.postgres_pool_group.clone());
        let review = review_repository
            .add_review(id_transaction, id_subject, rating, comment)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

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
        id_profile: Uuid,
        id_item: i64,
        intent_action: PledgeIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge> {
        let pledge_repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = pledge_repository
            .add_pledge(id_profile, id_item, intent_action, message, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

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
        status: PledgeStatus,
    ) -> FieldResult<Pledge> {
        debug!("Updating pledge: id={}", id);
        let repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .update_pledge(id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
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
    ) -> FieldResult<ScheduleOpportunity> {
        debug!(
            "Adding schedule opportunity: schedule={}, opportunity={}",
            id_schedule, id_opportunity
        );
        let repository = ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let schedule_opportunity = repository
            .add_schedule_opportunity(id_schedule, id_opportunity)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
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
        window_start: time::OffsetDateTime,
        window_end: time::OffsetDateTime,
    ) -> FieldResult<ScheduleOpportunity> {
        debug!("Updating schedule opportunity: id={}", id);
        let repository = ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let schedule_opportunity = repository
            .update_schedule_opportunity(id, window_start, window_end)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
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
    ) -> FieldResult<Schedule> {
        debug!("Adding schedule: scheduled_for={}", scheduled_for);
        let repository = ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository
            .add_schedule(scheduled_for)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
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
        status: TransactionStatus,
        id_schedule: Option<i64>,
        id_location: Option<i64>,
    ) -> FieldResult<Transaction> {
        debug!("Adding transaction: pledge={}", id_pledge);
        let transaction_repo = TransactionRepository::new(self.postgres_pool_group.clone());

        let transaction = transaction_repo
            .add_transaction(id_pledge, status, id_schedule, id_location)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

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
        status: TransactionStatus,
    ) -> FieldResult<Transaction> {
        debug!("Updating transaction: id={}", id);
        let repository = TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository
            .update_transaction(id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
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
    pub async fn add_message(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_sender: Uuid,
        type_: MessageType,
        content: String,
    ) -> Result<Message> {
        let message_repository = MessageRepository::new(self.postgres_pool_group.clone());
        let message = message_repository
            .add_message(id_transaction, id_sender, type_, content)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(message)
    }
}
