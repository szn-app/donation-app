use log::debug;
use time::OffsetDateTime;

use crate::database::model::listing::{Media, MediaType};
use crate::database::sql::listing::media::{
    CREATE_MEDIA, DELETE_MEDIA, FIND_MEDIA, FIND_MEDIA_BY_ITEM, UPDATE_MEDIA, LIST_MEDIA,
};
use crate::server::connection::PostgresPool;
use std::error::Error;

pub struct MediaRepository {
    pool: PostgresPool,
}

impl MediaRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_item: i64,
        caption: Option<String>,
        url: String,
        type_: MediaType,
    ) -> Result<Media, Box<dyn Error + Send + Sync>> {
        debug!("Creating media for item {}", id_item);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(CREATE_MEDIA, &[&id_item, &caption, &url, &type_])
            .await?;
        Ok(Media::from(row))
    }

    pub async fn find(&self, id: i64) -> Result<Option<Media>, Box<dyn Error + Send + Sync>> {
        debug!("Getting media by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_MEDIA, &[&id]).await?;
        Ok(row.map(Media::from))
    }

    pub async fn find_by_item(
        &self,
        id_item: i64,
    ) -> Result<Vec<Media>, Box<dyn Error + Send + Sync>> {
        debug!("Getting media for item: {}", id_item);
        let client = self.pool.r.get().await?;
        let rows = client.query(FIND_MEDIA_BY_ITEM, &[&id_item]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        caption: Option<String>,
        url: String,
        type_: MediaType,
    ) -> Result<Media, Box<dyn Error + Send + Sync>> {
        debug!("Updating media {}", id);
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(UPDATE_MEDIA, &[&id, &caption, &url, &type_, &now])
            .await?;
        let media = row.map(Media::from).ok_or("No record found to update")?;

        Ok(media)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting media {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_MEDIA, &[&id]).await?;
        Ok(rows_affected > 0)
    }

    pub async fn list(&self) -> Result<Vec<Media>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all media");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_MEDIA, &[]).await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }
}
