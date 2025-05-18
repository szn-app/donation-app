pub mod test;
pub mod user;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql;

// TODO: implement limit, filter, sort and pagination arguments.

/// GraphQL Query Root
#[derive(async_graphql::MergedObject)]
pub struct Query(user::UserQuery, test::TestQuery);

impl Query {
    pub fn new(postgres_pool_group: PostgresPool) -> Self {
        Self(
            user::UserQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            test::TestQuery {
                postgres_pool_group: postgres_pool_group,
            },
        )
    }
}
