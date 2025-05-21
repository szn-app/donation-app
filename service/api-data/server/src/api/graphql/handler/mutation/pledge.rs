use crate::access_control::check_permission_for_subject;
use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::api::graphql::repository::pledge::PledgeRepository;
use crate::database::model::interaction::{Pledge, PledgeIntentAction, PledgeStatus};
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, Error, FieldResult, Object, Result};
use log;
use tracing::debug;
use tracing::instrument;
use uuid::Uuid;

pub struct PledgeMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PledgeMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_pledge(
        &self,
        ctx: &Context<'_>,
        id_profile: Uuid,
        id_item: i64,
        intent_action: PledgeIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge> {
        let pledge_repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = pledge_repository
            .add_pledge(id_profile, id_item, intent_action, message, status)
            .await?;

        Ok(pledge)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_pledge(
        &self,
        ctx: &Context<'_>,
        id: i64,
        status: PledgeStatus,
    ) -> FieldResult<Pledge> {
        debug!("Updating pledge: id={}", id);
        let repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .update_pledge(id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(pledge)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_pledge(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting pledge: id={}", id);
        let repository = PledgeRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_pledge(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
}
