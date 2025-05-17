use crate::database::model::listing::Category;
use crate::database::sql::{ADD_CATEGORY, GET_CATEGORIES, GET_CATEGORY_BY_ID, UPDATE_CATEGORY};
use anyhow::Result;
use deadpool_postgres::Pool;
use tokio_postgres::Row;
use tracing::debug;
use uuid::Uuid;

pub struct CategoryRepository {
    pool: Pool,
}

impl CategoryRepository {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        debug!("Getting all categories");
        let client = self.pool.get().await?;
        let rows = client.query(GET_CATEGORIES, &[]).await?;
        Ok(rows.into_iter().map(Category::from).collect())
    }

    pub async fn get_category_by_id(&self, id: i64) -> Result<Option<Category>> {
        debug!("Getting category by id: {}", id);
        let client = self.pool.get().await?;
        let row = client.query_opt(GET_CATEGORY_BY_ID, &[&id]).await?;
        Ok(row.map(Category::from))
    }

    pub async fn add_category(
        &self,
        title: &str,
        description: &str,
        category_parent: Option<i64>,
    ) -> Result<Category> {
        debug!("Adding category: {}", title);
        let client = self.pool.get().await?;
        let row = client
            .query_one(ADD_CATEGORY, &[&title, &description, &category_parent])
            .await?;
        Ok(Category::from(row))
    }

    pub async fn update_category(
        &self,
        id: i64,
        title: &str,
        description: &str,
        category_parent: Option<i64>,
    ) -> Result<Category> {
        debug!("Updating category: {}", id);
        let client = self.pool.get().await?;
        let row = client
            .query_one(
                UPDATE_CATEGORY,
                &[&id, &title, &description, &category_parent],
            )
            .await?;
        Ok(Category::from(row))
    }
}
