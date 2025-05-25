use time::OffsetDateTime;

use crate::database::model::listing::media::{Media, MediaType};
use crate::error::Error;
use crate::server::connection::PostgresPool;

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
    ) -> Result<Media, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                "INSERT INTO listing.media (id_item, caption, url, type, created_at) 
                 VALUES ($1, $2, $3, $4, $5) RETURNING *",
                &[&id_item, &caption, &url, &type_, &now],
            )
            .await?;
        Ok(Media::from(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Media>, Error> {
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                "SELECT * FROM listing.media WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(row.map(Media::from))
    }

    pub async fn get_by_item(&self, id_item: i64) -> Result<Vec<Media>, Error> {
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                "SELECT * FROM listing.media WHERE id_item = $1",
                &[&id_item],
            )
            .await?;
        Ok(rows.into_iter().map(Media::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        caption: Option<String>,
        url: String,
        type_: MediaType,
    ) -> Result<Option<Media>, Error> {
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                "UPDATE listing.media 
                 SET caption = $2, url = $3, type = $4
                 WHERE id = $1 RETURNING *",
                &[&id, &caption, &url, &type_],
            )
            .await?;
        Ok(row.map(Media::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                "DELETE FROM listing.media WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
