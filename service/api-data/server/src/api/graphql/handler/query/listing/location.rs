use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};

pub struct LocationQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl LocationQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_locations(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Location>> {
        let repository = repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let locations = repository.list().await.map_err(|e| e.to_string())?;
        Ok(locations)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_location(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Location>> {
        let repository = repository::listing::LocationRepository::new(self.postgres_pool_group.clone());
        let location = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(location)
    }
} 