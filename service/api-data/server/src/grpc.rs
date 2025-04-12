use shared_proto::proto;
use tonic::{
    transport::Server as TonicServer, Request as TonicRequest, Response as TonicResponse, Status,
};
use tonic_reflection::server::Builder as ReflectionBuilder;

// gRPC service implementation (moved outside functions for clarity)
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

        log::debug!("Adding {}", user_id);

        // TODO: Simulate database operation (replace with actual database logic)
        if user_id == "fail" {
            return Err(Status::internal("Failed to add user to database"));
        }

        let response = TonicResponse::new(proto::user_sync::AddUserResponse {});
        Ok(response)
    }
}

pub async fn start_grpc_server() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_addr = "0.0.0.0:8082".parse()?;

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
        .await?;

    Ok(())
}
