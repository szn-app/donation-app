use crate::api::graphql::access_control::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

pub struct ReviewQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ReviewQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_reviews(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::Review>> {
        log::debug!("--> get_reviews @ graphql resolver");
        let repository =
            repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let reviews = repository.get_reviews().await.map_err(|e| e.to_string())?;
        Ok(reviews)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_review_by_id(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_subject: i64,
    ) -> FieldResult<Option<model::interaction::Review>> {
        log::debug!("--> get_review_by_id @ graphql resolver");
        let repository =
            repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let review = repository
            .get_review_by_id(id_transaction, id_subject)
            .await
            .map_err(|e| e.to_string())?;
        Ok(review)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_review_by_transaction_and_subject(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
        id_subject: i64,
    ) -> FieldResult<Option<model::interaction::Review>> {
        log::debug!("--> get_review_by_transaction_and_subject @ graphql resolver");
        let repository =
            repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let review = repository
            .get_review_by_transaction_and_subject(id_transaction, id_subject)
            .await
            .map_err(|e| e.to_string())?;
        Ok(review)
    }
}

pub struct PledgeQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PledgeQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_pledges(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::Pledge>> {
        log::debug!("--> get_pledges @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledges = repository.get_pledges().await.map_err(|e| e.to_string())?;
        Ok(pledges)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_pledge_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::interaction::Pledge>> {
        log::debug!("--> get_pledge_by_id @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .get_pledge_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(pledge)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_pledges_by_item(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
    ) -> FieldResult<Vec<model::interaction::Pledge>> {
        log::debug!("--> get_pledges_by_item @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledges = repository
            .get_pledges_by_item(id_item)
            .await
            .map_err(|e| e.to_string())?;
        Ok(pledges)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_pledges_by_profile(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
    ) -> FieldResult<Vec<model::interaction::Pledge>> {
        log::debug!("--> get_pledges_by_profile @ graphql resolver");
        let repository =
            repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledges = repository
            .get_pledges_by_profile(id_profile)
            .await
            .map_err(|e| e.to_string())?;
        Ok(pledges)
    }
}

pub struct ScheduleOpportunityQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleOpportunityQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_schedule_opportunities(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::ScheduleOpportunity>> {
        log::debug!("--> get_schedule_opportunities @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let schedule_opportunities = repository
            .get_schedule_opportunities()
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule_opportunities)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_schedule_opportunity_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::interaction::ScheduleOpportunity>> {
        log::debug!("--> get_schedule_opportunity_by_id @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(
            self.postgres_pool_group.clone(),
        );
        let schedule_opportunity = repository
            .get_schedule_opportunity_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule_opportunity)
    }
}

pub struct ScheduleQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ScheduleQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_schedules(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::Schedule>> {
        log::debug!("--> get_schedules @ graphql resolver");
        let repository =
            repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedules = repository
            .get_schedules()
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedules)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_schedule_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::interaction::Schedule>> {
        log::debug!("--> get_schedule_by_id @ graphql resolver");
        let repository =
            repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository
            .get_schedule_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(schedule)
    }
}

pub struct TransactionQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl TransactionQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_transactions(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::Transaction>> {
        log::debug!("--> get_transactions @ graphql resolver");
        let repository =
            repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transactions = repository
            .get_transactions()
            .await
            .map_err(|e| e.to_string())?;
        Ok(transactions)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_transaction_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::interaction::Transaction>> {
        log::debug!("--> get_transaction_by_id @ graphql resolver");
        let repository =
            repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository
            .get_transaction_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(transaction)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_transactions_by_pledge(
        &self,
        _ctx: &Context<'_>,
        id_pledge: i64,
    ) -> FieldResult<Vec<model::interaction::Transaction>> {
        log::debug!("--> get_transactions_by_pledge @ graphql resolver");
        let repository =
            repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transactions = repository
            .get_transactions_by_pledge(id_pledge)
            .await
            .map_err(|e| e.to_string())?;
        Ok(transactions)
    }
}

pub struct MessageQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MessageQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_messages(
        &self,
        _ctx: &Context<'_>,
    ) -> FieldResult<Vec<model::interaction::Message>> {
        log::debug!("--> get_messages @ graphql resolver");
        let repository =
            repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let messages = repository.get_messages().await.map_err(|e| e.to_string())?;
        Ok(messages)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_message_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::interaction::Message>> {
        log::debug!("--> get_message_by_id @ graphql resolver");
        let repository =
            repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let message = repository
            .get_message_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(message)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn get_messages_by_transaction(
        &self,
        _ctx: &Context<'_>,
        id_transaction: i64,
    ) -> FieldResult<Vec<model::interaction::Message>> {
        log::debug!("--> get_messages_by_transaction @ graphql resolver");
        let repository =
            repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let messages = repository
            .get_messages_by_transaction(id_transaction)
            .await
            .map_err(|e| e.to_string())?;
        Ok(messages)
    }
}
