use crate::database::model::listing::Item;
use crate::database::sql::{GET_ITEMS, GET_ITEM_BY_ID};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;

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
}
