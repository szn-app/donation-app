// https://docs.rs/tower/latest/tower/trait.Service.html

mod model;

use axum;
use env_logger;
use log;
use shared_proto::proto;
use std::env;
use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};
use tonic_reflection::server::Builder as ReflectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger with appropriate level based on environment
    let log_level = if cfg!(debug_assertions) {
        "debug"
    } else {
        "info"
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    let http_server = {
        let http_app = axum::Router::new()
            .merge(model::routes())
            // Add a fallback route for handling 404 Not Found
            .fallback(model::handle_not_found);

        let http_addr = "0.0.0.0:8081";
        let listener: tokio::net::TcpListener =
            tokio::net::TcpListener::bind(http_addr).await.unwrap();
        log::info!("HTTP server running on http://{}", http_addr);

        axum::serve(listener, http_app.into_make_service())
    };

    let grpc_server = {
        let grpc_addr = "0.0.0.0:8082".parse()?;

        // gRPC service implementation
        #[derive(Debug)]
        pub struct UserSyncService;

        #[tonic::async_trait]
        impl proto::user_sync::UserSync for UserSyncService {
            async fn add_user(
                &self,
                request: TonicRequest<proto::user_sync::AddUserRequest>,
            ) -> Result<TonicResponse<proto::user_sync::AddUserResponse>, Status> {
                let user_id = request.into_inner().user_id;

                if user_id.is_empty() {
                    return Err(Status::invalid_argument("User ID cannot be empty"));
                }

                //  TODO: Simulate database operation (replace with actual database logic) // For demonstration, assume the operation succeeds unless user_id is "fail"
                if user_id == "fail" {
                    return Err(Status::internal("Failed to add user to database"));
                }

                // Return an empty AddUserResponse on success
                let response = TonicResponse::new(proto::user_sync::AddUserResponse {});
                Ok(response)
            }
        }

        // Enable reflection with the file descriptor set (allows discovery - used by tools like grpcurl)
        let reflection_service = ReflectionBuilder::configure()
            .register_encoded_file_descriptor_set(proto::user_sync::DESCRIPTOR_SET)
            .register_encoded_file_descriptor_set(proto::test::DESCRIPTOR_SET)
            .build_v1()?;

        log::info!("gRPC server running on http://{}", grpc_addr);

        TonicServer::builder()
            .add_service(proto::user_sync::UserSyncServer::new(UserSyncService))
            .add_service(proto::test::GreeterServer::new(proto::test::GreeterService))
            .add_service(reflection_service)
            .serve(grpc_addr)
    };

    tokio::select! {
        result = http_server => result.map_err(Into::<Box<dyn std::error::Error>>::into),
        result = grpc_server => result.map_err(Into::<Box<dyn std::error::Error>>::into)
    }
}
