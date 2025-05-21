use async_graphql::Result;
use tracing::instrument;

use crate::access_control::check_permission_for_subject;
use crate::api::graphql::guard::{auth, AuthorizeUser};
use crate::database::model::listing::{Collection, CollectionType, Publish};
use crate::database::repository::collection::{CollectionRepository, PublishRepository};
use crate::database::repository::listing::collection::CollectionRepository;
use crate::server::connection::PostgresPool;
use crate::server::connection::{KetoChannelGroup, PostgresPool};
use async_graphql::{self, Context, Error, FieldResult, Object, Result};
use log;
use tracing::debug;
use uuid::Uuid;

pub struct CollectionMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CollectionMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_collection(
        &self,
        ctx: &Context<'_>,
        id_profile: String,
        title: String,
        description: Option<String>,
    ) -> Result<Collection> {
        let collection_repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = collection_repository
            .add_collection(id_profile, title, description)
            .await?;

        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update_collection(
        &self,
        ctx: &Context<'_>,
        id: i64,
        title: Option<String>,
        description: Option<String>,
    ) -> FieldResult<Collection> {
        debug!("Updating collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository
            .update_collection(id, title, description)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(collection)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete_collection(&self, ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting collection: id={}", id);
        let repository = CollectionRepository::new(self.postgres_pool_group.clone());
        repository
            .delete_collection(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn add_publish(
        &self,
        ctx: &Context<'_>,
        created_by: String,
        id_collection: i32,
        id_item: i32,
        note: Option<String>,
        position: i32,
    ) -> Result<Publish> {
        let publish_repository = PublishRepository::new(self.postgres_pool_group.clone());
        let publish = publish_repository
            .add_publish(created_by, id_collection, id_item, note, position)
            .await?;

        Ok(publish)
    }
}
