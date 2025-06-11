use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::interaction::{Pledge, PledgeIntentAction, PledgeStatus};
use crate::database::repository::interaction::pledge::PledgeRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use tracing::debug;
use uuid::Uuid;

pub struct PledgeMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PledgeMutation {
    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn create(
        &self,
        _ctx: &Context<'_>,
        id_profile: Uuid,
        id_item: i64,
        intent_action: PledgeIntentAction,
        message: Option<String>,
        status: PledgeStatus,
    ) -> Result<Pledge> {
        let pledge_repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = pledge_repository
            .create(id_profile, id_item, intent_action, message, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(pledge)
    }

    #[graphql(
        guard = "AuthorizeUser::group_admin_guard()",
        directive = auth::apply(Some("required_authorization".to_string()))
    )]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        status: PledgeStatus,
    ) -> FieldResult<Pledge> {
        debug!("Updating pledge: id={}", id);
        let repository = PledgeRepository::new(self.postgres_pool_group.clone());
        let pledge = repository
            .update(id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(pledge)
    }
} 