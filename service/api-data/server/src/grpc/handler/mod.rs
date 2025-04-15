use crate::server::connection::PostgresPool;
use shared_proto::proto::{self, user_sync::UserSync};
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};

// gRPC service implementation (moved outside functions for clarity)
#[derive(Debug)]
pub struct UserSyncService {
    postgres_pool_group: PostgresPool,
}

impl UserSyncService {
    pub fn new(postgres_pool_group: PostgresPool) -> Self {
        Self {
            postgres_pool_group,
        }
    }
}

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

        let client = self
            .postgres_pool_group
            .rw
            .get()
            .await
            .map_err(|e| Status::internal(format!("Failed to get DB connection: {}", e)))?;

        let _ = client
            .execute("INSERT INTO users (kratos_id) VALUES ($1)", &[&user_id])
            .await
            .map_err(|e| Status::internal(format!("DB error: {}", e)))?;

        let response = TonicResponse::new(proto::user_sync::AddUserResponse {});
        Ok(response)
    }
}
