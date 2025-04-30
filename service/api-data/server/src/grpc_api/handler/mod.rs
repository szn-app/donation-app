use crate::server::connection::PostgresPool;
use retry::{delay::Exponential, retry};
use shared_proto::proto::{self, user_sync::UserSync};
use std::time::Duration;
use tokio::time::sleep;
use tonic::{Request as TonicRequest, Response as TonicResponse, Status};
use uuid;

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

        let user_id = uuid::Uuid::parse_str(&user_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid UUID format: {}", e)))?;

        let client = {
            let mut last_err = None;
            let delay = Exponential::from_millis(100).take(3);

            let result = {
                let mut client_result = None;

                for d in delay {
                    match self.postgres_pool_group.rw.get().await {
                        Ok(client) => {
                            client_result = Some(client);
                            break;
                        }
                        Err(e) => {
                            last_err = Some(Status::internal(format!(
                                "Failed to get DB connection: {}",
                                e
                            )));
                            sleep(d).await;
                        }
                    }
                }

                client_result.ok_or_else(|| {
                    last_err.unwrap_or_else(|| {
                        Status::internal("Unknown error retrieving DB connection")
                    })
                })
            };

            result
        }?;

        let _ = client
            .execute("INSERT INTO user.account (id) VALUES ($1)", &[&user_id])
            .await
            .map_err(|e| {
                log::error!("gRPC database error: {}", e);
                Status::internal(format!("DB error: {}", e))
            })?;

        let response = TonicResponse::new(proto::user_sync::AddUserResponse {});
        Ok(response)
    }
}
