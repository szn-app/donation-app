pub mod interaction;
pub mod listing;
pub mod user;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql;
use async_graphql::{Context, Object, Result};

/// GraphQL Mutation Root
#[derive(async_graphql::MergedObject)]
pub struct Mutation(
    user::AccountMutation,
    user::CommitteeMutation,
    user::CommunityMutation,
    user::ProfileMutation,
    listing::CategoryMutation,
    listing::LocationMutation,
    listing::ItemMutation,
    listing::CollectionMutation,
    listing::MediaMutation,
    listing::PublishMutation,
    interaction::ReviewMutation,
    interaction::PledgeMutation,
    interaction::ScheduleOpportunityMutation,
    interaction::ScheduleMutation,
    interaction::TransactionMutation,
    interaction::MessageMutation,
);

impl Mutation {
    pub fn new(postgres_pool_group: PostgresPool) -> Self {
        Self(
            user::AccountMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::CommitteeMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::CommunityMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::ProfileMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::CategoryMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::LocationMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::ItemMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::CollectionMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::MediaMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::PublishMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ReviewMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::PledgeMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ScheduleOpportunityMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ScheduleMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::TransactionMutation {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::MessageMutation {
                postgres_pool_group,
            },
        )
    }
}
