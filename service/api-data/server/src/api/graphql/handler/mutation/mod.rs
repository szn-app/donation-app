pub mod user;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql;

/// GraphQL Mutation Root
#[derive(async_graphql::MergedObject)]
pub struct Mutation(
    user::AccountMutation,
    user::CommitteeMutation,
    user::CommunityMutation,
    user::ProfileMutation,
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
                postgres_pool_group,
            },
        )
    }
}
