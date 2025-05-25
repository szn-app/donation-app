use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model::listing::Category;
use crate::database::repository::listing::category::CategoryRepository;
use crate::server::connection::PostgresPool;
use async_graphql::{Context, Error, FieldResult, Object, Result};

pub struct CategoryMutation {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CategoryMutation {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    pub async fn create(
        &self,
        _ctx: &Context<'_>,
        name: String,
        description: String,
        parent_id: Option<i64>,
    ) -> Result<Category> {
        let category_repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = category_repository
            .create(&name, &description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn update(
        &self,
        _ctx: &Context<'_>,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        parent_id: Option<i64>,
    ) -> FieldResult<Category> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        let category = repository
            .update(id, name, description, parent_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(category)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn delete(&self, _ctx: &Context<'_>, id: i64) -> FieldResult<bool> {
        let repository = CategoryRepository::new(self.postgres_pool_group.clone());
        repository
            .delete(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(true)
    }
} 