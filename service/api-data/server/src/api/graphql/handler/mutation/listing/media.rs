use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::listing::{Media, MediaType};
use crate::database::repository::listing::media::MediaRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};
use tracing::debug;

pub struct MediaMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MediaMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        url: String,
        media_type: MediaType,
        position: i32,
    ) -> Result<Media> {
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = media_repository
            .create(id_item, None, url.to_string(), media_type)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        url: Option<String>,
        media_type: Option<MediaType>,
        position: Option<i32>,
    ) -> FieldResult<Media> {
        debug!("Updating media: id={}", id);
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = repository
            .update(id, None, url.unwrap_or_default(), media_type.unwrap_or_default())
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        debug!("Deleting media: id={}", id);
        let repository = MediaRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
} 