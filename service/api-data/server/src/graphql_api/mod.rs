pub mod access_constrol;
pub mod handler;
pub mod service;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{
    http::{GraphQLPlaygroundConfig, GraphiQLSource},
    Schema,
};
use async_graphql_axum::GraphQL;
use axum::{
    body, http,
    response::{Html, IntoResponse},
    routing::{get, post_service},
    Router,
};
use service::GlobalContext;
use std::{convert::Infallible, sync::Arc};

// - wrappers over schema components: mechanisms provided for permission checks, input validation, etc. https://async-graphql.github.io/async-graphql/en/utilities.html
// - graphql middlewares: https://async-graphql.github.io/async-graphql/en/extensions.html

pub fn routes(postgres_pool_group: PostgresPool, keto_channel_group: KetoChannelGroup) -> Router {
    let global_context = GlobalContext {
        keto_channel_group: keto_channel_group.clone(),
    }; // accessible through graphql resolver context

    let q = handler::Query {
        postgres_pool_group: postgres_pool_group.clone(), // pass context as instance value
    };

    let m = handler::Mutation {
        postgres_pool_group: postgres_pool_group, // pass context as instance value
    };

    let s = handler::EmptySubscription;

    let graphql_impl_schema = Schema::build(q, m, s).data(global_context).finish();

    Router::new()
        .route(
            "/graphql",
            // modified async_graphql_axum default GraphQL Axum service handle
            post_service(service::GraphQL::new(graphql_impl_schema)),
        )
        .route("/graphql-ide-1", get(ide_graphiql_handler))
        .route("/graphql-ide-2", get(ide_graphql_playground)) // Serve the GraphQL Playground
}

// GraphiQL UI explorer interface
// TODO: upgrade to fix loading issue https://github.com/async-graphql/async-graphql/issues
async fn ide_graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

/// GraphQL Playground
async fn ide_graphql_playground() -> impl IntoResponse {
    Html(async_graphql::http::playground_source(
        GraphQLPlaygroundConfig::new("/graphql"),
    ))
}
