use crate::database::model::listing::{
    Item, ItemCondition, ItemIntentAction, ItemStatus, ItemType,
};
use crate::database::sql::listing::item::{
    ADD_ITEM, DELETE_ITEM, GET_ITEMS, GET_ITEMS_BY_CATEGORY, GET_ITEMS_BY_PROFILE, GET_ITEM_BY_ID,
    UPDATE_ITEM,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct ItemRepository {
    pool: PostgresPool,
}

impl ItemRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, Box<dyn Error>> {
        debug!("Getting all items");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_ITEMS, &[]).await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn get_item_by_id(&self, id: i64) -> Result<Option<Item>, Box<dyn Error>> {
        debug!("Getting item by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_ITEM_BY_ID, &[&id]).await?;
        Ok(row.map(Item::from))
    }

    pub async fn get_items_by_profile(
        &self,
        id_profile: Uuid,
    ) -> Result<Vec<Item>, Box<dyn Error>> {
        debug!("Getting items by profile: {}", id_profile);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_ITEMS_BY_PROFILE, &[&id_profile]).await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn get_items_by_category(
        &self,
        id_category: i64,
    ) -> Result<Vec<Item>, Box<dyn Error>> {
        debug!("Getting items by category: {}", id_category);
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_ITEMS_BY_CATEGORY, &[&id_category]).await?;
        Ok(rows.into_iter().map(Item::from).collect())
    }

    pub async fn add_item(
        &self,
        item_type: ItemType,
        intent_action: ItemIntentAction,
        status: ItemStatus,
        title: &str,
        description: &str,
        category: i64,
        condition: ItemCondition,
        location: Option<i64>,
        created_by: Uuid,
    ) -> Result<Item, Box<dyn Error>> {
        debug!("Adding item: {}", title);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                ADD_ITEM,
                &[
                    &item_type,
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

    pub async fn update_item(
        &self,
        id: i64,
        item_type: Option<ItemType>,
        intent_action: Option<ItemIntentAction>,
        status: Option<ItemStatus>,
        title: Option<String>,
        description: Option<String>,
        category: Option<i64>,
        condition: Option<ItemCondition>,
        location: Option<i64>,
    ) -> Result<Item, Box<dyn Error>> {
        debug!("Updating item: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(
                UPDATE_ITEM,
                &[
                    &id,
                    &item_type,
                    &intent_action,
                    &status,
                    &title,
                    &description,
                    &category,
                    &condition,
                    &location,
                ],
            )
            .await?;
        Ok(Item::from(row))
    }

    pub async fn delete_item(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting item: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_ITEM, &[&id]).await?;
        Ok(())
    }
}
