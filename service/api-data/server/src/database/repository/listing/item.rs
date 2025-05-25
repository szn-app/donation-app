use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::model::listing::{
    Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType,
};
use crate::error::Error;
use crate::server::connection::PostgresPool;

pub struct ItemRepository {
    pool: PostgresPool,
}

impl ItemRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        type_: ItemType,
        intent_action: ItemIntentAction,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: ItemCondition,
        location: Option<i64>,
        created_by: Option<Uuid>,
    ) -> Result<Item, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_one(
                "INSERT INTO listing.item (
                    type, intent_action, status, title, description, 
                    category, condition, location, views_count, is_reported,
                    created_at, created_by
                ) VALUES (
                    $1, $2, 'draft', $3, $4, $5, $6, $7, 0, false, $8, $9
                ) RETURNING *",
                &[
                    &type_, &intent_action, &title, &description,
                    &category, &condition, &location, &now, &created_by
                ],
            )
            .await?;
        Ok(Item::from(row))
    }

    pub async fn get_by_id(&self, id: i64) -> Result<Option<Item>, Error> {
        let client = self.pool.r.get().await?;
        let row = client
            .query_opt(
                "SELECT * FROM listing.item WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(row.map(Item::from))
    }

    pub async fn get_by_created_by(&self, created_by: Uuid) -> Result<Vec<Item>, Error> {
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                "SELECT * FROM listing.item WHERE created_by = $1",
                &[&created_by],
            )
            .await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn get_by_category(&self, category: i64) -> Result<Vec<Item>, Error> {
        let client = self.pool.r.get().await?;
        let rows = client
            .query(
                "SELECT * FROM listing.item WHERE category = $1",
                &[&category],
            )
            .await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: ItemCondition,
        location: Option<i64>,
        status: ItemStatus,
    ) -> Result<Option<Item>, Error> {
        let client = self.pool.rw.get().await?;
        let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                "UPDATE listing.item 
                 SET title = $2, description = $3, category = $4,
                     condition = $5, location = $6, status = $7,
                     updated_at = $8
                 WHERE id = $1 RETURNING *",
                &[
                    &id, &title, &description, &category,
                    &condition, &location, &status, &now
                ],
            )
            .await?;
        Ok(row.map(Item::from))
    }

    pub async fn increment_views(&self, id: i64) -> Result<Option<Item>, Error> {
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                "UPDATE listing.item 
                 SET views_count = views_count + 1
                 WHERE id = $1 RETURNING *",
                &[&id],
            )
            .await?;
        Ok(row.map(Item::from))
    }

    pub async fn report(&self, id: i64) -> Result<Option<Item>, Error> {
        let client = self.pool.rw.get().await?;
        let row = client
            .query_opt(
                "UPDATE listing.item 
                 SET is_reported = true
                 WHERE id = $1 RETURNING *",
                &[&id],
            )
            .await?;
        Ok(row.map(Item::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Error> {
        let client = self.pool.rw.get().await?;
        let rows_affected = client
            .execute(
                "DELETE FROM listing.item WHERE id = $1",
                &[&id],
            )
            .await?;
        Ok(rows_affected > 0)
    }
}
