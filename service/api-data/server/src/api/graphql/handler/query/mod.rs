pub mod interaction;
pub mod listing;
pub mod test;
pub mod user;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql;

// TODO: implement limit, filter, sort and pagination arguments.

/// GraphQL Query Root
#[derive(async_graphql::MergedObject)]
pub struct Query(
    user::AccountQuery,
    user::CommitteeQuery,
    user::CommunityQuery,
    user::ProfileQuery,
    listing::CategoryQuery,
    listing::LocationQuery,
    listing::ItemQuery,
    listing::CollectionQuery,
    listing::MediaQuery,
    listing::PublishQuery,
    interaction::ReviewQuery,
    interaction::PledgeQuery,
    interaction::ScheduleOpportunityQuery,
    interaction::ScheduleQuery,
    interaction::TransactionQuery,
    interaction::MessageQuery,
    test::TestQuery,
);

impl Query {
    pub fn new(postgres_pool_group: PostgresPool) -> Self {
        Self(
            user::AccountQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::CommitteeQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::CommunityQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            user::ProfileQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::CategoryQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::LocationQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::ItemQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::CollectionQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::MediaQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            listing::PublishQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ReviewQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::PledgeQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ScheduleOpportunityQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::ScheduleQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::TransactionQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            interaction::MessageQuery {
                postgres_pool_group: postgres_pool_group.clone(),
            },
            test::TestQuery {
                postgres_pool_group,
            },
        )
    }
}
