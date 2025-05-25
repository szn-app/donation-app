use crate::api::graphql::guard::AuthorizeUser;
use crate::database::model;
use crate::database::repository;
use crate::server::connection::PostgresPool;
use async_graphql::{self, Context, FieldResult, Object};
use log;

pub struct MessageQuery {
    pub postgres_pool_group: PostgresPool,
}

#[async_graphql::Object]
impl MessageQuery {
    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn list_messages(&self, ctx: &Context<'_>) -> FieldResult<Vec<model::Message>> {
        log::debug!("--> list_messages @ graphql resolver");
        let repository = repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let messages = repository.list().await.map_err(|e| e.to_string())?;
        Ok(messages)
    }

    #[graphql(guard = "AuthorizeUser {
            namespace: \"Group\".to_string(),
            object: \"admin\".to_string(),
            relation: \"member\".to_string()
        }")]
    async fn find_message(&self, ctx: &Context<'_>, id: i64) -> FieldResult<Option<model::Message>> {
        log::debug!("--> find_message @ graphql resolver");
        let repository = repository::interaction::MessageRepository::new(self.postgres_pool_group.clone());
        let message = repository.find(id).await.map_err(|e| e.to_string())?;
        Ok(message)
    }
} 