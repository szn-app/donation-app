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
use async_graphql::{self, Context, Error, FieldResult, Object};
use log;
use tracing::debug;
use tracing::instrument;
use uuid;

pub struct AccountMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountMutation {
    /// Create a new account with explicit ID
    /// Note: ID must be provided and will not be auto-generated
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create_account(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
        remarks: Option<String>,
    ) -> Result<model::user::Account> {
        log::debug!("--> create_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.create(id, remarks).await?;
        Ok(account)
    }

    /// Update an account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn update_account(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
        remarks: Option<String>,
    ) -> Result<Option<model::user::Account>> {
        log::debug!("--> update_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.update(id, remarks).await?;
        Ok(account)
    }

    /// Delete an account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_account(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<bool> {
        log::debug!("--> delete_account @ graphql resolver");
        let repository = AccountRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| e.to_string())?;
        Ok(result)
    }
}

pub struct ProfileMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ProfileMutation {
    /// Add a new profile
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create_profile(
        &self,
        _ctx: &Context<'_>,
        id_account: uuid::Uuid,
        name: String,
        email: String,
        phone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<model::user::Profile> {
        let profile_repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = profile_repository
            .create(
                name,
                Some(email),
                Some(ProfileType::Individual),
                id_account,
                id_account
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(profile)
    }

    /// Update a profile
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_profile(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: String,
        description: Option<String>,
        type_: Option<ProfileType>,
    ) -> FieldResult<Option<model::user::Profile>> {
        log::debug!("--> update_profile @ graphql resolver");
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .update(id, name, description, type_)
            .await
            .map_err(|e| e.to_string())?;
        Ok(profile)
    }

    /// Delete a profile
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_profile(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        log::debug!("--> delete_profile @ graphql resolver");
        let repository = ProfileRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| e.to_string())?;
        Ok(result)
    }
}

pub struct CommunityMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommunityMutation {
    /// Add a new community
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create_community(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        avatar_url: Option<String>,
        created_by: uuid::Uuid,
    ) -> Result<model::user::Community> {
        let community_repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = community_repository
            .create(
                name,
                Some(description),
                CommunityType::Solo, // default type
                created_by,
                created_by
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

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
        title: String,
        description: Option<String>,
        type_: CommunityType,
    ) -> FieldResult<Option<model::user::Community>> {
        log::debug!("--> update_community @ graphql resolver");
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .update(id, title, description, type_)
            .await
            .map_err(|e| e.to_string())?;
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
        let repository = CommunityRepository::new(self.postgres_pool_group.clone());
        let result = repository.delete(id).await.map_err(|e| e.to_string())?;
        Ok(result)
    }
}

pub struct CommitteeMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommitteeMutation {
    /// Add a new committee
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
    ) -> Result<model::user::Committee> {
        let committee_repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = committee_repository
            .create(
                id_profile,
                id_community,
                member_role,
            )
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(committee)
    }

    /// Update a committee's role
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_committee_role(
        &self,
        _ctx: &Context<'_>,
        id_profile: i64,
        id_community: i64,
        member_role: CommitteeRole,
    ) -> FieldResult<Option<model::user::Committee>> {
        log::debug!("--> update_committee_role @ graphql resolver");
        let repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .update(id_profile, id_community, member_role)
            .await
            .map_err(|e| e.to_string())?;
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
        id_profile: i64,
        id_community: i64,
    ) -> FieldResult<bool> {
        log::debug!("--> delete_committee @ graphql resolver");
        let repository = CommitteeRepository::new(self.postgres_pool_group.clone());
        let result = repository
            .delete(id_profile, id_community)
            .await
            .map_err(|e| e.to_string())?;
        Ok(result)
    }
}
