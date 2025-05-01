pub mod mutation_resolver;
pub mod query_resolver;

use crate::server::connection::PostgresPool;
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::GraphQL;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use mutation_resolver::MutationResolver;
use query_resolver::QueryResolver;
use std::{convert::Infallible, sync::Arc};

// context-based appraoch to share data with resolvers
#[derive(Clone)]
pub struct Context {}

pub fn routes(postgres_pool_group: PostgresPool) -> Router {
    let context = Context {};

    let query_resolver = QueryResolver {
        postgres_pool_group: postgres_pool_group.clone(), // pass context as instance value
    };
    let mutation_resolver = MutationResolver {
        postgres_pool_group: postgres_pool_group, // pass context as instance value
    };
    let subscription_resolver = EmptySubscription;

    let graphql_impl_schema =
        Schema::build(query_resolver, mutation_resolver, subscription_resolver)
            .data(context)
            .finish();

    Router::new().route(
        "/graphql",
        get(graphiql_handler).post_service(GraphQL::new(graphql_impl_schema)),
    )
}

// GraphiQL UI explorer interface
async fn graphiql_handler() -> impl IntoResponse {
    Html(GraphiQLSource::build().endpoint("/graphql").finish())
}
