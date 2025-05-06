use super::super::service::DataContext;
use crate::database::model;
use crate::database::query;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;
// {
// directive @auth(rule: Rule) on FIELD_DEFINITION

// enum Rule {
//   IS_AUTHOR
// }

// type Post {
//   authorId: ID!
//   body: String @auth(rule: IS_AUTHOR)
// }

// }

/// GraphQL Query Root
pub struct Query {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl Query {
    /// Get all accounts
    async fn accounts(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        // let c = ctx.data::<super::Context>()?; // EXMAPLE

        let account_list = query::user::AccountRepository::get_accounts(&self.postgres_pool_group)
            .await
            .map_err(|e| e.to_string())?;

        Ok(account_list)
    }

    // for debugging purposes
    async fn dummyTest(&self, ctx: &Context<'_>) -> FieldResult<Vec<std::string::String>> {
        log::debug!("--> dummyTest @ graphql resolver");
        // let c = ctx.data::<super::Context>()?; // EXMAPLE

        Ok(vec![
            "word1".to_string(),
            "word2".to_string(),
            "word3".to_string(),
        ])
    }

    // for debugging purposes
    async fn dummyTestRequestHeader(&self, ctx: &Context<'_>) -> FieldResult<String> {
        log::debug!("--> dummyTestRequestHeader @ graphql resolver");

        let c = ctx.data::<DataContext>()?;
        dbg!(c);

        Ok("message here".to_string())
    }

    // for debugging purposes
    async fn dummyTestSecure(&self, ctx: &Context<'_>) -> FieldResult<model::test::Test> {
        log::debug!("--> dummyTestSecure @ graphql resolver");

        let c = ctx.data::<DataContext>()?;
        dbg!(c);

        Ok(model::test::Test {
            message: "secret message here".to_string(),
        })
    }
}
