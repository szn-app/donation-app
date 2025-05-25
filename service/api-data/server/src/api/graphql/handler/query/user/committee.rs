use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

pub struct CommitteeQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommitteeQuery {
    /// Get all committees
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_committees(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Committee>> {
        log::debug!("--> committees @ graphql resolver");
        let repository = repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committees = repository.list().await.map_err(|e| e.to_string())?;
        Ok(committees)
    }

    /// Get committee by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_committee(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<Option<model::user::Committee>> {
        log::debug!("--> committee_by_id @ graphql resolver");
        let repository = repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(committee)
    }
} 