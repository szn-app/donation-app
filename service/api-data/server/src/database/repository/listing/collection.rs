use time::OffsetDateTime;

use crate::database::model::listing::collection::{Collection, CollectionType, CollectionVisibility};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct CollectionRepository {
    pool: PostgresPool,
}

impl CollectionRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_community: Option<i64>,
        title: Option<String>,
        visibility: CollectionVisibility,
        type_: Option<CollectionType>,
        position: i32,
    ) -> Result<Collection, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                "INSERT INTO listing.collection (
                    id_community, title, visibility, type, position, created_at
                ) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
                &[&id_community, &title, &visibility, &type_, &position, &now],
            )
            .await?;
        Ok(Collection::from(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Collection>, Error> {
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                "SELECT * FROM listing.collection WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(row.map(Collection::from))
    }

    pub async fn get_by_community(&self, id_community: i64) -> Result<Vec<Collection>, Error> {
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                "SELECT * FROM listing.collection WHERE id_community = $1 ORDER BY position",
                &[&id_community],
            )
            .await?;
        Ok(rows.into_iter().map(Collection::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        title: Option<String>,
        visibility: CollectionVisibility,
        type_: Option<CollectionType>,
        position: i32,
    ) -> Result<Option<Collection>, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                "UPDATE listing.collection 
                 SET title = $2, visibility = $3, type = $4, position = $5, updated_at = $6
                 WHERE id = $1 RETURNING *",
                &[&id, &title, &visibility, &type_, &position, &now],
            )
            .await?;
        Ok(row.map(Collection::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                "DELETE FROM listing.collection WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
