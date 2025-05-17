use crate::database::model::listing::{Media, MediaType};
use crate::database::sql::{
    ADD_MEDIA, DELETE_MEDIA, GET_MEDIA, GET_MEDIA_BY_ID, GET_MEDIA_BY_ITEM,
};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct MediaRepository {
    pool: Pool,
}

impl MediaRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_media(&self) -> Result<Vec<Media>> {
        debug!("Getting all media");
        let client = self.pool.get().await?;
        let rows = client.query(GET_MEDIA, &[]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn get_media_by_id(&self, id: i64) -> Result<Option<Media>> {
        debug!("Getting media by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_MEDIA_BY_ID, &[&id]).await?;
        Ok(row.map(Media::from))
    }

    pub async fn get_media_by_item(&self, id_item: i64) -> Result<Vec<Media>> {
        debug!("Getting media by item: {}", id_item);
        let client = self.pool.get().await?;
        let rows = client.query(GET_MEDIA_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn add_media(
        &self,
        id_item: i64,
        type_: MediaType,
        url: &str,
        created_by: Uuid,
    ) -> Result<Media> {
        debug!("Adding media for item: {}", id_item);
        let client = self.pool.get().await?;
        let row = client
            .query_one(ADD_MEDIA, &[&id_item, &type_, &url, &created_by])
            .await?;
        Ok(Media::from(row))
    }

    pub async fn delete_media(&self, id: i64) -> Result<()> {
        debug!("Deleting media: {}", id);
        let client = self.pool.get().await?;
        client.execute(DELETE_MEDIA, &[&id]).await?;
        Ok(())
    }
}
