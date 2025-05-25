use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::model::listing::publish::Publish;
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct PublishRepository {
    pool: PostgresPool,
}

impl PublishRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
        added_by: Option<Uuid>,
    ) -> Result<Publish, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                "INSERT INTO listing.publish (id_item, id_collection, note, position, added_by, posted_on) 
                 VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
                &[&id_item, &id_collection, &note, &position, &added_by, &now],
            )
            .await?;
        Ok(Publish::from(row))
    }

    pub async fn get_by_item_and_collection(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<Option<Publish>, Error> {
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                "SELECT * FROM listing.publish WHERE id_item = $1 AND id_collection = $2",
                &[&id_item, &id_collection],
            )
            .await?;
        Ok(row.map(Publish::from))
    }

    pub async fn get_by_collection(&self, id_collection: i64) -> Result<Vec<Publish>, Error> {
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                "SELECT * FROM listing.publish WHERE id_collection = $1 ORDER BY position",
                &[&id_collection],
            )
            .await?;
        Ok(rows.into_iter().map(Publish::from).collect())
    }

    pub async fn update(
        &self,
        id_item: i64,
        id_collection: i64,
        note: Option<String>,
        position: i32,
    ) -> Result<Option<Publish>, Error> {
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                "UPDATE listing.publish 
                 SET note = $3, position = $4
                 WHERE id_item = $1 AND id_collection = $2 
                 RETURNING *",
                &[&id_item, &id_collection, &note, &position],
            )
            .await?;
        Ok(row.map(Publish::from))
    }

    pub async fn delete(
        &self,
        id_item: i64,
        id_collection: i64,
    ) -> Result<bool, Error> {
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                "DELETE FROM listing.publish 
                 WHERE id_item = $1 AND id_collection = $2",
                &[&id_item, &id_collection],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
