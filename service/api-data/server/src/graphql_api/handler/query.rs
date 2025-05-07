use super::super::access_constrol::check::check_permission_for_subject;
use super::super::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::query;
use crate::server::connection::{KetoChannelGroup, PostgresPool};

use async_graphql::{self, Context, Error, ErrorExtensions, FieldResult, Object};
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;

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

        Ok("message here".to_string())
    }

    // for debugging purposes
    async fn dummyTestSecure(&self, ctx: &Context<'_>) -> FieldResult<model::test::Test> {
        log::debug!("--> dummyTestSecure @ graphql resolver");

        let c = ctx.data::<DataContext>()?;

        if let Some(user_id) = &c.user_id {
            if user_id == "anonymous" {
                return Err(
                    async_graphql::Error::new("Unauthorized").extend_with(|_, e| {
                        e.set("code", "UNAUTHORIZED");
                        e.set("status", 401); // UNAUTHORIZED 401: didn't provide proper authentication; FORBIDDEN 403: authenticated but not authorized
                    }),
                );
            }
        }

        Ok(model::test::Test {
            secureMessage: "secret message here".to_string(),
        })
    }

    // for debugging purposes
    async fn dummyTestSecureGuard(&self, ctx: &Context<'_>) -> FieldResult<model::test::Test> {
        log::debug!("--> dummyTestSecure @ graphql resolver");

        let c = ctx.data::<DataContext>()?;
        let g = ctx.data::<GlobalContext>()?;

        let r =
            check_permission_for_subject(g.keto_channel_group.write.clone(), "n", "o", "r", "s")
                .await;
        dbg!(r);

        // if let Some(user_id) = &c.user_id {
        //     if user_id == "anonymous" {
        //         return Err(
        //             async_graphql::Error::new("Unauthorized").extend_with(|_, e| {
        //                 e.set("code", "UNAUTHORIZED");
        //                 e.set("status", 401); // UNAUTHORIZED 401: didn't provide proper authentication; FORBIDDEN 403: authenticated but not authorized
        //             }),
        //         );
        //     }
        // }

        Ok(model::test::Test {
            secureMessage: "secret message here".to_string(),
        })
    }
}
