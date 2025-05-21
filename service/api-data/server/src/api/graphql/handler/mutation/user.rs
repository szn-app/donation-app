use crate::access_control::check_permission_for_subject;
use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::model::user::{
    Account, Committee, CommitteeRole, Community, CommunityType, Profile, ProfileType,
};
use crate::database::repository;
use crate::database::repository::user::{
    AccountRepository, CommitteeRepository, CommunityRepository, ProfileRepository,
};
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::Result;
use async_graphql::{self, Context, FieldResult, Object};
use log;
use tracing::debug;
use tracing::instrument;
use uuid;

pub struct AccountMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_account(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> Result<model::user::Account> {
        let account_repository = AccountRepository::new(self.postgres_pool_group.clone());
        let account = account_repository.add_account(id).await?;

        Ok(account)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_account(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<bool> {
        debug!("Deleting account: id={}", id);
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_account(id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct ProfileMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ProfileMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_profile(
        &self,
        _ctx: &Context<'_>,
        id_account: uuid::Uuid,
        name: String,
        description: Option<String>,
        profile_type: Option<ProfileType>,
    ) -> Result<model::user::Profile> {
        let profile_repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = profile_repository
            .add_profile(id_account, &name, description, profile_type)
            .await?;

        Ok(profile)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_profile(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
        name: Option<String>,
        description: Option<String>,
        profile_type: Option<ProfileType>,
    ) -> FieldResult<model::user::Profile> {
        debug!("Updating profile: {}", id);
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .update_profile(id, name, description, profile_type)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(profile)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_profile(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<bool> {
        debug!("Deleting profile: id={}", id);
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_profile(id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}

pub struct CommunityMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommunityMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_community(
        &self,
        _ctx: &Context<'_>,
        title: String,
        description: String,
        community_type: CommunityType,
        owner: uuid::Uuid,
        created_by: uuid::Uuid,
    ) -> Result<model::user::Community> {
        let community_repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = community_repository
            .add_community(&title, &description, community_type, owner, created_by)
            .await?;

        Ok(community)
    }

    /// Update a community
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_community(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        community_type: Option<CommunityType>,
    ) -> FieldResult<model::user::Community> {
        debug!("Updating community: {}", id);
        let community = CommunityRepository::new(self.postgres_pool_group.clone())
            .update_community(id, title, description, community_type)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(community)
    }

    /// Delete a community
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_community(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        log::debug!("--> delete_community @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_community(id)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}

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
    pub async fn add_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> Result<model::user::Committee> {
        let committee_repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = committee_repository
            .add_committee(id_profile, id_community, member_role)
            .await?;

        Ok(committee)
    }

    /// Update committee role
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_committee_role(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> FieldResult<model::user::Committee> {
        debug!(
            "Updating committee role for profile: {} and community: {}",
            id_profile, id_community
        );
        let committee = CommitteeRepository::new(self.postgres_pool_group.clone())
            .update_committee_role(id_profile, id_community, member_role)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(committee)
    }

    /// Delete a committee
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
    ) -> FieldResult<bool> {
        debug!(
            "Deleting committee for profile: {} and community: {}",
            id_profile, id_community
        );
        let repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_committee(id_profile, id_community)
            .await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        Ok(true)
    }
}
