use log::debug;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::database::model::listing::{
    Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType,
};
use crate::database::sql::listing::item::{
    CREATE_ITEM, DELETE_ITEM, FIND_ITEM, FIND_ITEMS_BY_CATEGORY, INCREMENT_VIEWS, LIST_ITEMS,
    REPORT_ITEM, UPDATE_ITEM,
};
use crate::server::connection::PostgresPool;
use std::error::Error;

pub struct ItemRepository {
    pool: PostgresPool,
}

impl ItemRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        variant: Option<ItemType>,
        intent_action: Option<ItemIntentAction>,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: Option<ItemCondition>,
        location: Option<i64>,
        created_by: Option<Uuid>,
        status: Option<ItemStatus>,
    ) -> Result<Item, Box<dyn Error + Send + Sync>> {
        debug!("Creating new item with title: {:?}", title);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                CREATE_ITEM,
                &[
                    &variant,
                    &intent_action,
                    &status,
                    &title,
                    &description,
                    &category,
                    &condition,
                    &location,
                    &created_by,
                ],
            )
            .await?;
        Ok(Item::from(row))
    }

    pub async fn find(&self, id: i64) -> Result<Option<Item>, Box<dyn Error + Send + Sync>> {
        debug!("Getting item by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_ITEM, &[&id]).await?;
        Ok(row.map(Item::from))
    }

    pub async fn find_by_category(
        &self,
        category: i64,
        status: Option<ItemStatus>,
    ) -> Result<Vec<Item>, Box<dyn Error + Send + Sync>> {
        debug!("Getting items by category: {}", category);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(FIND_ITEMS_BY_CATEGORY, &[&category, &status])
            .await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn update(
        &self,
        id: i64,
        status: Option<ItemStatus>,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: Option<ItemCondition>,
        location: Option<i64>,
    ) -> Result<Item, Box<dyn Error + Send + Sync>> {
        debug!("Updating item {} with title: {:?}", id, title);
        let client = self.pool.rw.get().await?;
        // let now = OffsetDateTime::now_utc();
        let row = client
            .query_opt(
                UPDATE_ITEM,
                &[
                    &id,
                    &status,
                    &title,
                    &description,
                    &category,
                    &condition,
                    &location,
                    // &now,
                ],
            )
            .await?;

        let item = row.map(Item::from).ok_or("No record found to update")?;

        Ok(item)
    }

    pub async fn increment_views(
        &self,
        id: i64,
    ) -> Result<Option<Item>, Box<dyn Error + Send + Sync>> {
        debug!("Incrementing views for item: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client.query_opt(INCREMENT_VIEWS, &[&id]).await?;
        Ok(row.map(Item::from))
    }

    pub async fn report(&self, id: i64) -> Result<Option<Item>, Box<dyn Error + Send + Sync>> {
        debug!("Reporting item: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client.query_opt(REPORT_ITEM, &[&id]).await?;
        Ok(row.map(Item::from))
    }

    pub async fn delete(&self, id: i64) -> Result<bool, Box<dyn Error + Send + Sync>> {
        debug!("Deleting item: {}", id);
        let client = self.pool.rw.get().await?;
        let rows_affected = client.execute(DELETE_ITEM, &[&id]).await?;
        Ok(rows_affected > 0)
    }

    pub async fn list(
        &self,
        status: Option<ItemStatus>,
    ) -> Result<Vec<Item>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all items");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_ITEMS, &[&status]).await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }
}
