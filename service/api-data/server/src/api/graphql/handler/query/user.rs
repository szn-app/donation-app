use crate::api::graphql::access_constrol::{
    check::check_permission_for_subject,
    guard::{auth, AuthorizeUser},
};
use crate::api::graphql::service::{DataContext, GlobalContext};
use crate::database::model;
use crate::database::repository;
use crate::server::connection::{KetoChannelGroup, PostgresPool};

use async_graphql::{self, Context, Error, ErrorExtensions, FieldResult, Object}; // note: `graphql` attribute is processed by async_graphql macros
use deadpool_postgres::Pool;
use http::HeaderMap;
use log;
use time;
use uuid;

pub struct AccountQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl AccountQuery {
    /// Get all accounts
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn accounts(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let accounts = repository.get_accounts().await.map_err(|e| e.to_string())?;
        Ok(accounts)
    }

    /// Get account by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn account_by_id(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<Option<model::user::Account>> {
        log::debug!("--> account_by_id @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository
            .get_account_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(account)
    }
}

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
    async fn committees(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Committee>> {
        log::debug!("--> committees @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committees = repository
            .get_committees()
            .await
            .map_err(|e| e.to_string())?;
        Ok(committees)
    }

    /// Get committee by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn committee_by_id(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
    ) -> FieldResult<Option<model::user::Committee>> {
        log::debug!("--> committee_by_id @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .get_committee_by_id(id_profile, id_community)
            .await
            .map_err(|e| e.to_string())?;
        Ok(committee)
    }

    /// Get committee by profile and community
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn committee_by_profile_and_community(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
        id_community: i64,
    ) -> FieldResult<Option<model::user::Committee>> {
        log::debug!("--> committee_by_profile_and_community @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committee = repository
            .get_committee_by_profile_and_community(id_profile, id_community)
            .await
            .map_err(|e| e.to_string())?;
        Ok(committee)
    }

    /// Get committees by community
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn committees_by_community(
        &self,
        _ctx: &Context<'_>,
        id_community: i64,
    ) -> FieldResult<Vec<model::user::Committee>> {
        log::debug!("--> committees_by_community @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committees = repository
            .get_committees_by_community(id_community)
            .await
            .map_err(|e| e.to_string())?;
        Ok(committees)
    }

    /// Get committees by profile
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn committees_by_profile(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
    ) -> FieldResult<Vec<model::user::Committee>> {
        log::debug!("--> committees_by_profile @ graphql resolver");
        let repository =
            repository::user::CommitteeRepository::new(self.postgres_pool_group.clone());
        let committees = repository
            .get_committees_by_profile(id_profile)
            .await
            .map_err(|e| e.to_string())?;
        Ok(committees)
    }
}

pub struct CommunityQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CommunityQuery {
    /// Get all communities
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn communities(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Community>> {
        log::debug!("--> communities @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let communities = repository
            .get_communities()
            .await
            .map_err(|e| e.to_string())?;
        Ok(communities)
    }

    /// Get community by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn community_by_id(
        &self,
        _ctx: &Context<'_>,
        id: i64,
    ) -> FieldResult<Option<model::user::Community>> {
        log::debug!("--> community_by_id @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository
            .get_community_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(community)
    }

    /// Get communities by profile
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn communities_by_profile(
        &self,
        _ctx: &Context<'_>,
        id_profile: uuid::Uuid,
    ) -> FieldResult<Vec<model::user::Community>> {
        log::debug!("--> communities_by_profile @ graphql resolver");
        let repository =
            repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let communities = repository
            .get_communities_by_profile(id_profile)
            .await
            .map_err(|e| e.to_string())?;
        Ok(communities)
    }
}

pub struct ProfileQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl ProfileQuery {
    /// Get all profiles
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn profiles(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Profile>> {
        log::debug!("--> profiles @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profiles = repository.get_profiles().await.map_err(|e| e.to_string())?;
        Ok(profiles)
    }

    /// Get profile by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn profile_by_id(
        &self,
        _ctx: &Context<'_>,
        id: uuid::Uuid,
    ) -> FieldResult<Option<model::user::Profile>> {
        log::debug!("--> profile_by_id @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .get_profile_by_id(id)
            .await
            .map_err(|e| e.to_string())?;
        Ok(profile)
    }

    /// Get profile by account
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn profile_by_account(
        &self,
        _ctx: &Context<'_>,
        id_account: uuid::Uuid,
    ) -> FieldResult<Option<model::user::Profile>> {
        log::debug!("--> profile_by_account @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository
            .get_profile_by_account(id_account)
            .await
            .map_err(|e| e.to_string())?;
        Ok(profile)
    }
}
