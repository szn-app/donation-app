use crate::api::graphql::guard::{auth, AuthorizeUser};
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
    async fn list_accounts(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Account>> {
        log::debug!("--> accounts @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let accounts = repository.list().await.map_err(|e| e.to_string())?;
        Ok(accounts)
    }

    /// Get account by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_account(&self, _ctx: &Context<'_>, id: uuid::Uuid) -> FieldResult<Option<model::user::Account>> {
        log::debug!("--> account_by_id @ graphql resolver");
        let repository = repository::user::AccountRepository::new(self.postgres_pool_group.clone());
        let account = repository.find(id).await.map_err(|e| e.to_string())?;
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
    async fn list_communities(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Community>> {
        log::debug!("--> communities @ graphql resolver");
        let repository = repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let communities = repository.list().await.map_err(|e| e.to_string())?;
        Ok(communities)
    }

    /// Get community by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_community(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::user::Community>> {
        log::debug!("--> community_by_id @ graphql resolver");
        let repository = repository::user::CommunityRepository::new(self.postgres_pool_group.clone());
        let community = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(community)
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
    async fn list_profiles(&self, _ctx: &Context<'_>) -> FieldResult<Vec<model::user::Profile>> {
        log::debug!("--> profiles @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profiles = repository.list().await.map_err(|e| e.to_string())?;
        Ok(profiles)
    }

    /// Get profile by ID
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_profile(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::user::Profile>> {
        log::debug!("--> profile_by_id @ graphql resolver");
        let repository = repository::user::ProfileRepository::new(self.postgres_pool_group.clone());
        let profile = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(profile)
    }
}
