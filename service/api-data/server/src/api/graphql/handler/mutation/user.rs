use crate::api::graphql::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, FieldResult, Object};
use log;
use uuid;

pub struct AccountMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountMutation {
    /// Create a new account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn add_account(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<model::user::Account> {
        log::debug!("--> add_account @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository
            .add_account(id)
            .await
            .map_err(|e| e.to_string())?;
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
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_account(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
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
    async fn add_committee(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
        role: String,
    ) -> FieldResult<model::user::Committee> {
        log::debug!("--> add_committee @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .add_committee(id_profile, id_community, &role)
            .await
            .map_err(|e| e.to_string())?;
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
        role: String,
    ) -> FieldResult<model::user::Committee> {
        log::debug!("--> update_committee_role @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .update_committee_role(id_profile, id_community, &role)
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
        id_profile: uuid::Uuid,
        id_community: i64,
    ) -> FieldResult<bool> {
        log::debug!("--> delete_committee @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_committee(id_profile, id_community)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
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
    async fn add_community(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        created_by: uuid::Uuid,
    ) -> FieldResult<model::user::Community> {
        log::debug!("--> add_community @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .add_community(&name, &description, created_by)
            .await
            .map_err(|e| e.to_string())?;
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
        name: Option<String>,
        description: Option<String>,
    ) -> FieldResult<model::user::Community> {
        log::debug!("--> update_community @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .update_community(id, name, description)
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
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_community(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(true)
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
    async fn add_profile(
        &self,
        _ctx: &Context<'_>,
        id_account: uuid::Uuid,
        name: String,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> FieldResult<model::user::Profile> {
        log::debug!("--> add_profile @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .add_profile(id_account, &name, bio, avatar_url)
            .await
            .map_err(|e| e.to_string())?;
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
        id: uuid::Uuid,
        name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> FieldResult<model::user::Profile> {
        log::debug!("--> update_profile @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .update_profile(id, name, bio, avatar_url)
            .await
            .map_err(|e| e.to_string())?;
        Ok(profile)
    }
}
