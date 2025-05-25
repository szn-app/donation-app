use async_graphql::Result;
use tracing::instrument;

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
    pub async fn create_collection(
        &self,
        _ctx: &Context<'_>,
        id_community: i64,
        title: String,
        visibility: CollectionVisibility,
        type_: CollectionType,
        position: i32,
    ) -> Result<Collection> {
        let collection_repository = CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = collection_repository
            .create(id_community, &title, visibility, type_, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

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
    pub async fn create_publish(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        created_by: Uuid,
    ) -> Result<Publish> {
        let publish_repository = PublishRepository::new(self.postgres_pool_group.clone());
        let publish = publish_repository
            .create(id_item, id_collection, note.as_deref(), position, created_by)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(publish)
    }
}
