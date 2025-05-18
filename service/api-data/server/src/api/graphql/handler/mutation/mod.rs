pub mod user;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql;

/// GraphQL Mutation Root
#[derive(async_graphql::MergedObject)]
pub struct Mutation(user::UserMutation);

impl Mutation {
    pub fn new(postgres_pool_group: PostgresPool) -> Self {
        Self(user::UserMutation {
            postgres_pool_group: postgres_pool_group.clone(),
        })
    }
}
