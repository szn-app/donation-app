use crate::server::connection::KetoChannelGroup;
/**
 * Custom Axum service - modified from async-graphql-context@v7.0.16
 * - Added extractor function to allow for request data propagation to GraphQL context resolvers
 *
 * https://github.com/async-graphql/async-graphql/blob/75a9d14e8f45176a32bac7f458534c05cabd10cc/integrations/axum/src/query.rs
 */
use async_graphql::{
    http::{create_multipart_mixed_stream, is_accept_multipart_mixed},
    Data, Executor,
};
use async_graphql_axum::{rejection, GraphQLBatchRequest, GraphQLRequest, GraphQLResponse};
use axum::{
    body::{self, Body, HttpBody},
    extract::FromRequest,
    http::{self, Extensions, HeaderMap},
    response::IntoResponse,
    BoxError,
};
use bytes::Bytes;
use futures_util::{future::BoxFuture, StreamExt};
use std::{
    convert::Infallible,
    task::{Context, Poll},
    time::Duration,
};
use tower;

// context-based appraoch to share data accross resolvers
#[derive(Clone)]
pub struct GlobalContext {
    pub keto_channel_group: KetoChannelGroup,
}

// request specific context
#[derive(Clone, Debug)]
pub struct DataContext {
    pub user_id: Option<String>,
}

impl DataContext {
    fn from_request(req: &http::Request<impl body::HttpBody>) -> Self {
        // extract header
        let user_id = req
            .headers()
            .get("X-User")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        DataContext { user_id }
    }
}

/// A GraphQL service.
#[derive(Clone)]
pub struct GraphQL<E> {
    executor: E,
}

impl<E> GraphQL<E>
where
    E: Executor,
{
    pub fn new(executor: E) -> Self {
        Self { executor }
    }
}

impl<B, E> tower::Service<http::Request<B>> for GraphQL<E>
where
    B: HttpBody<Data = Bytes> + Send + 'static,
    B::Data: Into<Bytes>,
    B::Error: Into<BoxError>,
    E: Executor,
{
    type Response = http::Response<Body>;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: http::Request<B>) -> Self::Future {
        let executor = self.executor.clone();

        Box::pin(async move {
            // Convert directly from request components to DataContext
            let context = DataContext::from_request(&req);

            let req = req.map(Body::new);

            let is_accept_multipart_mixed = req
                .headers()
                .get("accept")
                .and_then(|v| v.to_str().ok())
                .map(is_accept_multipart_mixed)
                .unwrap_or_default();

            if is_accept_multipart_mixed {
                let req =
                    match GraphQLRequest::<rejection::GraphQLRejection>::from_request(req, &())
                        .await
                    {
                        Ok(req) => req,
                        Err(err) => return Ok(err.into_response()),
                    };
                let req = req.0.data(context);
                let stream = executor.execute_stream(req, None);
                let body = Body::from_stream(
                    create_multipart_mixed_stream(stream, Duration::from_secs(30))
                        .map(Ok::<_, std::io::Error>),
                );
                Ok(http::Response::builder()
                    .header("content-type", "multipart/mixed; boundary=graphql")
                    .body(body)
                    .expect("BUG: invalid response"))
            } else {
                let req = match GraphQLBatchRequest::<rejection::GraphQLRejection>::from_request(
                    req,
                    &(),
                )
                .await
                {
                    Ok(req) => req,
                    Err(err) => return Ok(err.into_response()),
                };
                let req = req.0.data(context);
                Ok(GraphQLResponse(executor.execute_batch(req).await).into_response())
            }
        })
    }
}
