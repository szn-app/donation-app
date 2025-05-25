use crate::database::model::listing::Category;
use crate::database::sql::listing::category::{
    CREATE_CATEGORY, DELETE_CATEGORY, LIST_CATEGORIES, FIND_CATEGORIES_BY_PARENT, FIND_CATEGORY,
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

    pub async fn list(&self) -> Result<Vec<Category>, Box<dyn Error + Send + Sync>> {
        debug!("Getting all categories");
        let client = self.pool.r.get().await?;
        let rows = client.query(LIST_CATEGORIES, &[]).await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    pub async fn find(&self, id: i64) -> Result<Option<Category>, Box<dyn Error + Send + Sync>> {
        debug!("Getting category by id: {}", id);
        let client = self.pool.r.get().await?;
        let row = client.query_opt(FIND_CATEGORY, &[&id]).await?;
        Ok(row.map(Category::from))
    }

    pub async fn find_by_parent(
        &self,
        parent_id: Option<i64>,
    ) -> Result<Vec<Category>, Box<dyn Error + Send + Sync>> {
        debug!("Getting categories by parent: {:?}", parent_id);
        let client = self.pool.r.get().await?;
        let rows = client
            .query(FIND_CATEGORIES_BY_PARENT, &[&parent_id])
            .await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    pub async fn create(
        &self,
        name: &str,
        description: &str,
        parent_id: Option<i64>,
    ) -> Result<Category, Box<dyn Error + Send + Sync>> {
        debug!("Adding category: {}", name);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(CREATE_CATEGORY, &[&name, &description, &parent_id])
            .await?;
        Ok(Category::from(row))
    }

    pub async fn update(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> Result<Category, Box<dyn Error + Send + Sync>> {
        debug!("Updating category: {}", id);
        let client = self.pool.rw.get().await?;
        let row = client
            .query_one(UPDATE_CATEGORY, &[&id, &name, &description, &parent_id])
            .await?;
        Ok(Category::from(row))
    }

    pub async fn delete(&self, id: i64) -> Result<(), Box<dyn Error + Send + Sync>> {
        debug!("Deleting category: {}", id);
        let client = self.pool.rw.get().await?;
        client.execute(DELETE_CATEGORY, &[&id]).await?;
        Ok(())
    }
}
