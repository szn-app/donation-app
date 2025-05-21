use crate::database::model::listing::{Media, MediaType};
use crate::database::sql::listing::media::{
    ADD_MEDIA, DELETE_MEDIA, GET_MEDIA, GET_MEDIA_BY_ID, GET_MEDIA_BY_ITEM, UPDATE_MEDIA,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct MediaRepository {
    pool: PostgresPool,
}

impl MediaRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_media(&self) -> Result<Vec<Media>, Box<dyn Error>> {
        debug!("Getting all media");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_MEDIA, &[]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn get_media_by_id(&self, id: i64) -> Result<Option<Media>, Box<dyn Error>> {
        debug!("Getting media by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_MEDIA_BY_ID, &[&id]).await?;
        Ok(row.map(Media::from))
    }

    pub async fn get_media_by_item(&self, id_item: i64) -> Result<Vec<Media>, Box<dyn Error>> {
        debug!("Getting media by item: {}", id_item);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_MEDIA_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn add_media(
        &self,
        id_item: i64,
        url: &str,
        media_type: MediaType,
        position: i32,
    ) -> Result<Media, Box<dyn Error>> {
        debug!("Adding media for item: {}", id_item);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_MEDIA, &[&id_item, &url, &media_type, &position])
            .await?;
        Ok(Media::from(row))
    }

    pub async fn update_media(
        &self,
        id: i64,
        url: Option<String>,
        media_type: Option<MediaType>,
        position: Option<i32>,
    ) -> Result<Media, Box<dyn Error>> {
        debug!("Updating media: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_MEDIA, &[&id, &url, &media_type, &position])
            .await?;
        Ok(Media::from(row))
    }

    pub async fn delete_media(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting media: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_MEDIA, &[&id]).await?;
        Ok(())
    }
}
