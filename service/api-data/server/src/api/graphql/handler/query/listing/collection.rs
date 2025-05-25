use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct CollectionQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl CollectionQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_collections(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Collection>> {
        let repository = repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collections = repository.list().await.map_err(|e| e.to_string())?;
        Ok(collections)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_collection(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Collection>> {
        let repository = repository::listing::CollectionRepository::new(self.postgres_pool_group.clone());
        let collection = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(collection)
    }
} 