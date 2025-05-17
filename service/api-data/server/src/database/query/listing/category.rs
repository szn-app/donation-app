use crate::database::model::listing::Category;
use crate::database::sql::listing::category::{
    ADD_CATEGORY, DELETE_CATEGORY, GET_CATEGORIES, GET_CATEGORIES_BY_PARENT, GET_CATEGORY_BY_ID,
    UPDATE_CATEGORY,
};
use crate::server::connection::PostgresPool;
use deadpool_postgres::PoolError;
use std::error::Error;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CategoryRepository {
    pool: PostgresPool,
}

impl CategoryRepository {
    pub fn new(pool: PostgresPool) -> Self {
        Self { pool }
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>, Box<dyn Error>> {
        debug!("Getting all categories");
        let client = self.pool.r.get().await?;
        let rows = client.query(GET_CATEGORIES, &[]).await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    pub async fn get_category_by_id(&self, id: i64) -> Result<Option<Category>, Box<dyn Error>> {
        debug!("Getting category by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(GET_CATEGORY_BY_ID, &[&id]).await?;
        Ok(row.map(Category::from))
    }

    pub async fn get_categories_by_parent(
        &self,
        parent_id: Option<i64>,
    ) -> Result<Vec<Category>, Box<dyn Error>> {
        debug!("Getting categories by parent: {:?}", parent_id);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(GET_CATEGORIES_BY_PARENT, &[&parent_id])
            .await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    pub async fn add_category(
        &self,
        name: &str,
        description: &str,
        parent_id: Option<i64>,
    ) -> Result<Category, Box<dyn Error>> {
        debug!("Adding category: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(ADD_CATEGORY, &[&name, &description, &parent_id])
            .await?;
        Ok(Category::from(row))
    }

    pub async fn update_category(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> Result<Category, Box<dyn Error>> {
        debug!("Updating category: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_CATEGORY, &[&id, &name, &description, &parent_id])
            .await?;
        Ok(Category::from(row))
    }

    pub async fn delete_category(&self, id: i64) -> Result<(), Box<dyn Error>> {
        debug!("Deleting category: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_CATEGORY, &[&id]).await?;
        Ok(())
    }
}
