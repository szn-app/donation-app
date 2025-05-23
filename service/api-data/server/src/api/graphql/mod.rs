pub mod guard;
pub mod handler;
pub mod service;

pub use handler::EmptySubscription;
pub use handler::Mutation;
pub use handler::Query;

use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{
    http::{GraphQLPlaygroundConfig, GraphiQLSource},
    Schema,
};
use axum::{
    body,
    response::{Html, IntoResponse},
    routing::{get, post_service},
    Router,
};
// use async_graphql_axum::GraphQL; // custom GraphQL builder is used instead.
use crate::server::http::handle_not_found;
use http;
use service::{GlobalContext, GraphQL};
use std::{convert::Infallible, sync::Arc};
use tower_http;

// - wrappers over schema components: mechanisms provided for permission checks, input validation, etc. https://async-graphql.github.io/async-graphql/en/utilities.html
// - graphql middlewares: https://async-graphql.github.io/async-graphql/en/extensions.html

pub fn routes(
    app_endpoint: &str,
    postgres_pool_group: PostgresPool,
    keto_channel_group: KetoChannelGroup,
) -> Router {
    let graphql_router = create_graphql_router(postgres_pool_group, keto_channel_group);

    // allow calling api subdomain from root domain
    let cors_layer = tower_http::cors::CorsLayer::new()
        // NOTE: wild card origin headers do not work well in all browsers
        .allow_origin(app_endpoint.parse::<http::header::HeaderValue>().unwrap())
        // NOTE: methods should be explicitely mentioned (tower_http::cors::Any won't work on browsers) when credentials are passed (e.g. Authorization headers)
        .allow_methods([
            http::Method::OPTIONS,
            http::Method::POST,
            http::Method::GET,
            http::Method::HEAD,
            // http::Method::PUT,
            // http::Method::DELETE,
            // http::Method::CONNECT,
            // http::Method::PATCH,
            // http::Method::TRACE,
        ])
        // IMPORTANT: headers should be explicitely mentioned for browsers not to block the CORS request.
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
        ]);

    Router::new()
        .merge(graphql_router)
        // .route("ws", ....) // TODO: websocket subscriptions
        .route("/ide-1", get(ide_graphiql_handler))
        .route("/ide-2", get(ide_graphql_playground)) // Serve the GraphQL Playground
        .fallback(handle_not_found) // redundant as already handled in parent router (but can still be useful for mocking and testing)
        .layer(cors_layer) // when verifying CORS, make sure to check directly exposed calls and those rerouted through the gateway, as well as in the browser network section also the React request should properly include cookies and headers, etc. // IMPOTRANT: Gateway & Oathkeeper access-control must allow OPTIONS method
}

fn create_graphql_router(
    postgres_pool_group: PostgresPool,
    keto_channel_group: KetoChannelGroup,
) -> Router {
    let global_context = GlobalContext {
        keto_channel_group: keto_channel_group.clone(),
    };

    let q = handler::Query::new(postgres_pool_group.clone());
    let m = handler::Mutation::new(postgres_pool_group);
    let s = handler::EmptySubscription;

    let graphql_impl_schema = Schema::build(q, m, s).data(global_context).finish();

    let graphql_service = tower::ServiceBuilder::new().service(GraphQL::new(graphql_impl_schema)); // layer must be applied within service to influence internal middlewares

    Router::new().route("/", post_service(graphql_service.clone()))
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
