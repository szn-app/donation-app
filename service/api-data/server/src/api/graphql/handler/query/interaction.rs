use crate::api::graphql::guard::{auth, AuthorizeUser};
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
    async fn list_reviews(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::interaction::Review>> {
        log::debug!("--> list_reviews @ graphql resolver");
        let repository = repository::interaction::ReviewRepository::new(self.postgres_pool_group.clone());
        let reviews = repository.list().await.map_err(|e| e.to_string())?;
        Ok(reviews)
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
    async fn list_pledges(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::interaction::Pledge>> {
        log::debug!("--> list_pledges @ graphql resolver");
        let repository = repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledges = repository.list().await.map_err(|e| e.to_string())?;
        Ok(pledges)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_pledge(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::interaction::Pledge>> {
        log::debug!("--> find_pledge @ graphql resolver");
        let repository = repository::interaction::PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(pledge)
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
    async fn list_schedule_opportunities(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::ScheduleOpportunity>> {
        log::debug!("--> list_schedule_opportunities @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let opportunities = repository.list().await.map_err(|e| e.to_string())?;
        Ok(opportunities)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_schedule_opportunity(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::ScheduleOpportunity>> {
        log::debug!("--> find_schedule_opportunity @ graphql resolver");
        let repository = repository::interaction::ScheduleOpportunityRepository::new(self.postgres_pool_group.clone());
        let opportunity = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(opportunity)
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
    async fn list_schedules(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Schedule>> {
        log::debug!("--> list_schedules @ graphql resolver");
        let repository = repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedules = repository.list().await.map_err(|e| e.to_string())?;
        Ok(schedules)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_schedule(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Schedule>> {
        log::debug!("--> find_schedule @ graphql resolver");
        let repository = repository::interaction::ScheduleRepository::new(self.postgres_pool_group.clone());
        let schedule = repository.find(id).await.map_err(|e| e.to_string())?;
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
    async fn list_transactions(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Transaction>> {
        log::debug!("--> list_transactions @ graphql resolver");
        let repository = repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transactions = repository.list().await.map_err(|e| e.to_string())?;
        Ok(transactions)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_transaction(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Transaction>> {
        log::debug!("--> find_transaction @ graphql resolver");
        let repository = repository::interaction::TransactionRepository::new(self.postgres_pool_group.clone());
        let transaction = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(transaction)
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
    async fn list_messages(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Message>> {
        log::debug!("--> list_messages @ graphql resolver");
        let repository = repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let messages = repository.list().await.map_err(|e| e.to_string())?;
        Ok(messages)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_message(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Message>> {
        log::debug!("--> find_message @ graphql resolver");
        let repository = repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let message = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(message)
    }
}
