use super::super::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use super::super::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::query;
use crate::server::connection::{KetoChannelGroup, PostgresPool};

use async_graphql::{self, Context, Error, ErrorExtensions, FieldResult, Object}; // note: `graphql` attribute is processed by async_graphql macros
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;
use time;

/// GraphQL Query Root
pub struct Query {
    pub postgres_pool_group: PostgresPool,
}

// TODO: implement limit, filter, sort and pagination arguments.

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

    async fn tests(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::test::Test>> {
        log::debug!("--> tests @ graphql resolver");
        // let c = ctx.data::<super::Context>()?; // EXMAPLE

        let test_list = query::test::TestRepository::get_tests(&self.postgres_pool_group)
            .await
            .map_err(|e| e.to_string())?;

        Ok(test_list)
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
            i: 1,
            s: "secret message here".to_string(),
            d: time::OffsetDateTime::now_local()?,
        })
    }

    // for debugging purposes
    async fn dummyTestSecurePermissionCheck(
        &self,
        ctx: &Context<'_>,
    ) -> FieldResult<model::test::Test> {
        log::debug!("--> dummyTestSecurePermissionCheck @ graphql resolver");

        let c = ctx.data::<DataContext>()?;
        let g = ctx.data::<GlobalContext>()?;

        let keto_client = g.keto_channel_group.write.clone();

        let user_id = c
            .user_id
            .as_ref()
            .ok_or("Not authenticated & no user info provided")?;

        match check_permission_for_subject(keto_client, "Test", "object", "relation", user_id).await
        {
            Ok(true) => println!("permitted"),
            Ok(false) => {
                return Err(async_graphql::Error::new(format!(
                    "Unauthorized (after keto permission check) for user {}",
                    user_id
                ))
                .extend_with(|_, e| {
                    e.set("code", "UNAUTHORIZED");
                    e.set("status", 401); // UNAUTHORIZED 401: didn't provide proper authentication; FORBIDDEN 403: authenticated but not authorized
                }));
            }

            Err(e) => {
                // Convert the error to a string representation
                let error_msg = format!("Permission check failed: {}", e);
                return Err(async_graphql::Error::new(error_msg));
            }
        };

        Ok(model::test::Test {
            i: 1,
            s: "secret message here".to_string(),
            d: time::OffsetDateTime::now_local()?,
        })
    }

    // for debugging purposes
    #[graphql(
        guard = "AuthorizeUser {
            namespace: \"Endpoint\".to_string(),
            object: \"k8s\".to_string(),
            relation: \"access\".to_string()
        }",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn dummyTestSecureGuard(&self, ctx: &Context<'_>) -> FieldResult<model::test::Test> {
        log::debug!("--> dummyTestSecureGuard @ graphql resolver");

        Ok(model::test::Test {
            i: 1,
            s: "secret message here".to_string(),
            d: time::OffsetDateTime::now_local()?,
        })
    }
}
