// TODO:

// use async_graphql::{EmptyMutation, EmptySubscription, Schema};
// use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
// use axum::{routing::get, Extension, Json, Router};
// use std::sync::Arc;

// use crate::{
//     graphql::QueryRoot, // You should define this somewhere
// };

// pub async fn graphql_handler(
//     schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
//     req: GraphQLRequest,
// ) -> GraphQLResponse {
//     schema.execute(req.into_inner()).await.into()
// }

// // Optional: Serve GraphQL Playground UI
// async fn graphql_playground() -> axum::response::Html<String> {
//     axum::response::Html(async_graphql::http::playground_source(
//         async_graphql::http::GraphQLPlaygroundConfig::new("/graphql"),
//     ))
// }
