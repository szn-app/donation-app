use crate::database::query::user;
use crate::server::connection::PostgresPool;
use log;
use shared_protobuf_webhook::proto::{self, user_sync::UserSync};
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};
use uuid::Uuid;

/// gRPC service implementation for user synchronization
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
impl UserSync for UserSyncService {
    async fn add_user(
        &self,
        request: TonicRequest<proto::user_sync::AddUserRequest>,
    ) -> Result<TonicResponse<proto::user_sync::AddUserResponse>, Status> {
        let user_id = request.into_inner().user_id;

        // Validate user ID
        if user_id.is_empty() {
            return Err(Status::invalid_argument("User ID cannot be empty"));
        }

        // Parse UUID
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid UUID format: {}", e)))?;

        log::debug!("Adding user with ID: {}", user_id);

        user::AccountRepository::add_account(&self.postgres_pool_group, user_uuid)
            .await
            .map_err(|e| Status::internal(format!("Database error: {}", e)))?;

        // Return success response
        Ok(TonicResponse::new(proto::user_sync::AddUserResponse {}))
    }
}
