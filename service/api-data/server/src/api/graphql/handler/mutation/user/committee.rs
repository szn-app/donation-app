use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::user::{Committee, CommitteeRole};
use crate::database::repository::user::CommitteeRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use log;

pub struct CommitteeMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommitteeMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<Committee> {
        let committee_repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = committee_repository
            .create(id_profile, id_community, member_role)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(committee)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> FieldResult<Option<Committee>> {
        log::debug!("--> update_committee_role @ graphql resolver");
        let repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .update(id_profile, id_community, member_role)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(committee)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: i64,
        id_community: i64,
    ) -> FieldResult<bool> {
        log::debug!("--> delete_committee @ graphql resolver");
        let repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let result = repository
            .delete(id_profile, id_community)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(result)
    }
} 