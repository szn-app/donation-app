use async_graphql::{Context, Error, Result};
use tracing::{debug, instrument};

use crate::api::graphql::repository::media::MediaRepository;
use crate::database::model::listing::Media;
use crate::server::auth::AuthorizeUser;
use crate::server::connection::PostgresPool;

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
    pub async fn create_media(
        &self,
        _ctx: &Context<'_>,
        id_item: i64,
        url: String,
        media_type: MediaType,
        position: i32,
    ) -> Result<Media> {
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());
        let media = media_repository
            .create(id_item, &url, media_type, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn update_media(
        &self,
        ctx: &Context<'_>,
        id: i64,
        url: Option<String>,
        media_type: Option<String>,
        position: Option<i32>,
    ) -> Result<Media> {
        debug!("Updating media: id={}", id);
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());

        let media = media_repository
            .update_media(id, url, media_type, position)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(media)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn delete_media(&self, ctx: &Context<'_>, id: i64) -> Result<bool> {
        debug!("Deleting media: id={}", id);
        let media_repository = MediaRepository::new(self.postgres_pool_group.clone());

        media_repository
            .delete_media(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(true)
    }
}
