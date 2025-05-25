use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct PublishQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl PublishQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_publishes(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Publish>> {
        let repository = repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publishes = repository.list().await.map_err(|e| e.to_string())?;
        Ok(publishes)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_publish(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Publish>> {
        let repository = repository::listing::PublishRepository::new(self.postgres_pool_group.clone());
        let publish = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(Some(publish))
    }
} 